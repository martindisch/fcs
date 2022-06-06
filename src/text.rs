use nom::{
    branch::alt,
    bytes::complete::{is_not, take},
    character::complete::char,
    combinator::recognize,
    multi::{count, many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Text {
    delimiter: char,
    pairs: HashMap<String, String>,
}

fn parse(input: &str) -> IResult<&str, Text> {
    let (input, delimiter) = take(1usize)(input)?;
    let delimiter = delimiter.chars().next().expect(
        "Since we consumed the first character, we know it'll be here",
    );

    let (input, pairs) = kv_pairs(input, delimiter)?;
    // TODO: replace escaped delimiters
    let pairs = pairs
        .into_iter()
        .map(|(key, value)| (key.to_uppercase(), value.to_string()))
        .collect();

    Ok((input, Text { delimiter, pairs }))
}

fn kv_pairs(input: &str, delimiter: char) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(char(delimiter), |i| kv_pair(i, delimiter))(input)
}

fn kv_pair(input: &str, delimiter: char) -> IResult<&str, (&str, &str)> {
    separated_pair(
        |i| undelimited_string(i, delimiter),
        char(delimiter),
        |i| undelimited_string(i, delimiter),
    )(input)
}

fn undelimited_string(input: &str, delimiter: char) -> IResult<&str, &str> {
    recognize(many1(alt((
        |i| not_delimiter(i, delimiter),
        |i| escaped_delimiter(i, delimiter),
    ))))(input)
}

fn not_delimiter(input: &str, delimiter: char) -> IResult<&str, &str> {
    is_not(&[delimiter][..])(input)
}

fn escaped_delimiter(input: &str, delimiter: char) -> IResult<&str, &str> {
    recognize(count(char(delimiter), 2))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_full() {
        let input = ",$key1,val,,ue1,$KEY2,,,value2,";
        assert_eq!(
            Ok((
                ",",
                Text {
                    delimiter: ',',
                    pairs: HashMap::from([
                        ("$KEY1".into(), "val,,ue1".into()),
                        ("$KEY2,,".into(), "value2".into())
                    ])
                }
            )),
            parse(input)
        );
    }

    #[test]
    fn kv_pairs_single() {
        let input = "ab,cd";
        assert_eq!(Ok(("", vec![("ab", "cd")])), kv_pairs(input, ','));
    }

    #[test]
    fn kv_pairs_multiple() {
        let input = "ab,cd,ef,gh";
        assert_eq!(
            Ok(("", vec![("ab", "cd"), ("ef", "gh")])),
            kv_pairs(input, ',')
        );
    }

    #[test]
    fn kv_pairs_multiple_overflow() {
        let input = "ab,cd,ef,gh,ij";
        assert_eq!(
            Ok((",ij", vec![("ab", "cd"), ("ef", "gh")])),
            kv_pairs(input, ',')
        );
    }

    #[test]
    fn kv_pairs_escaped() {
        let input = "ab,,,c,,d";
        assert_eq!(Ok(("", vec![("ab,,", "c,,d")])), kv_pairs(input, ','));
    }

    #[test]
    fn kv_pair_basic() {
        let input = "ab,cd";
        assert_eq!(Ok(("", ("ab", "cd"))), kv_pair(input, ','));
    }

    #[test]
    fn kv_pair_escaped_middle() {
        let input = "a,,b,cd";
        assert_eq!(Ok(("", ("a,,b", "cd"))), kv_pair(input, ','));
    }

    #[test]
    fn kv_pair_escaped_end() {
        let input = "ab,,,cd";
        assert_eq!(Ok(("", ("ab,,", "cd"))), kv_pair(input, ','));
    }

    #[test]
    fn kv_pair_escaped_overflow() {
        let input = "ab,cd,e";
        assert_eq!(Ok((",e", ("ab", "cd"))), kv_pair(input, ','));
    }

    #[test]
    fn undelimited_string_easy() {
        let input = "ab";
        assert_eq!(Ok(("", "ab")), undelimited_string(input, ','));
    }

    #[test]
    fn undelimited_string_escaped_start() {
        let input = ",,ab";
        assert_eq!(Ok(("", ",,ab")), undelimited_string(input, ','));
    }

    #[test]
    fn undelimited_string_escaped_middle() {
        let input = "a,,b";
        assert_eq!(Ok(("", "a,,b")), undelimited_string(input, ','));
    }

    #[test]
    fn undelimited_string_escaped_end() {
        let input = "ab,,";
        assert_eq!(Ok(("", "ab,,")), undelimited_string(input, ','));
    }

    #[test]
    fn undelimited_string_escaped_only() {
        let input = ",,";
        assert_eq!(Ok(("", ",,")), undelimited_string(input, ','));
    }

    #[test]
    fn undelimited_string_escaped_multiple() {
        let input = "a,,,,b";
        assert_eq!(Ok(("", "a,,,,b")), undelimited_string(input, ','));
    }

    #[test]
    fn undelimited_string_unescaped() {
        let input = "a,b";
        assert_eq!(Ok((",b", "a")), undelimited_string(input, ','));
    }

    #[test]
    fn undelimited_string_escaped_unescaped_one() {
        let input = "a,,,b";
        assert_eq!(Ok((",b", "a,,")), undelimited_string(input, ','));
    }

    #[test]
    fn not_delimiter_single() {
        let input = "a";
        assert_eq!(Ok(("", "a")), not_delimiter(input, ','));
    }

    #[test]
    fn not_delimiter_multiple() {
        let input = "ab";
        assert_eq!(Ok(("", "ab")), not_delimiter(input, ','));
    }

    #[test]
    fn not_delimiter_terminated() {
        let input = "ab,";
        assert_eq!(Ok((",", "ab")), not_delimiter(input, ','));
    }

    #[test]
    fn not_delimiter_followed() {
        let input = "ab,cd";
        assert_eq!(Ok((",cd", "ab")), not_delimiter(input, ','));
    }

    #[test]
    fn escaped_delimiter_single() {
        let input = ",,";
        assert_eq!(Ok(("", ",,")), escaped_delimiter(input, ','));
    }
}

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::char,
    combinator::recognize,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

fn kv_pairs(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(char(','), kv_pair)(input)
}

fn kv_pair(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(unseparated_string, char(','), unseparated_string)(input)
}

fn unseparated_string(input: &str) -> IResult<&str, &str> {
    recognize(many1(alt((not_separator, escaped_separator))))(input)
}

fn not_separator(input: &str) -> IResult<&str, &str> {
    is_not(",")(input)
}

fn escaped_separator(input: &str) -> IResult<&str, &str> {
    tag(",,")(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kv_pairs_single() {
        let input = "ab,cd";
        assert_eq!(Ok(("", vec![("ab", "cd")])), kv_pairs(input));
    }

    #[test]
    fn kv_pairs_multiple() {
        let input = "ab,cd,ef,gh";
        assert_eq!(
            Ok(("", vec![("ab", "cd"), ("ef", "gh")])),
            kv_pairs(input)
        );
    }

    #[test]
    fn kv_pairs_multiple_overflow() {
        let input = "ab,cd,ef,gh,ij";
        assert_eq!(
            Ok((",ij", vec![("ab", "cd"), ("ef", "gh")])),
            kv_pairs(input)
        );
    }

    #[test]
    fn kv_pairs_escaped() {
        let input = "ab,,,c,,d";
        assert_eq!(Ok(("", vec![("ab,,", "c,,d")])), kv_pairs(input));
    }

    #[test]
    fn kv_pair_basic() {
        let input = "ab,cd";
        assert_eq!(Ok(("", ("ab", "cd"))), kv_pair(input));
    }

    #[test]
    fn kv_pair_escaped_middle() {
        let input = "a,,b,cd";
        assert_eq!(Ok(("", ("a,,b", "cd"))), kv_pair(input));
    }

    #[test]
    fn kv_pair_escaped_end() {
        let input = "ab,,,cd";
        assert_eq!(Ok(("", ("ab,,", "cd"))), kv_pair(input));
    }

    #[test]
    fn kv_pair_escaped_overflow() {
        let input = "ab,cd,e";
        assert_eq!(Ok((",e", ("ab", "cd"))), kv_pair(input));
    }

    #[test]
    fn unseparated_string_easy() {
        let input = "ab";
        assert_eq!(Ok(("", "ab")), unseparated_string(input));
    }

    #[test]
    fn unseparated_string_escaped_start() {
        let input = ",,ab";
        assert_eq!(Ok(("", ",,ab")), unseparated_string(input));
    }

    #[test]
    fn unseparated_string_escaped_middle() {
        let input = "a,,b";
        assert_eq!(Ok(("", "a,,b")), unseparated_string(input));
    }

    #[test]
    fn unseparated_string_escaped_end() {
        let input = "ab,,";
        assert_eq!(Ok(("", "ab,,")), unseparated_string(input));
    }

    #[test]
    fn unseparated_string_escaped_only() {
        let input = ",,";
        assert_eq!(Ok(("", ",,")), unseparated_string(input));
    }

    #[test]
    fn unseparated_string_escaped_multiple() {
        let input = "a,,,,b";
        assert_eq!(Ok(("", "a,,,,b")), unseparated_string(input));
    }

    #[test]
    fn unseparated_string_unescaped() {
        let input = "a,b";
        assert_eq!(Ok((",b", "a")), unseparated_string(input));
    }

    #[test]
    fn unseparated_string_escaped_unescaped_one() {
        let input = "a,,,b";
        assert_eq!(Ok((",b", "a,,")), unseparated_string(input));
    }

    #[test]
    fn not_separator_single() {
        let input = "a";
        assert_eq!(Ok(("", "a")), not_separator(input));
    }

    #[test]
    fn not_separator_multiple() {
        let input = "ab";
        assert_eq!(Ok(("", "ab")), not_separator(input));
    }

    #[test]
    fn not_separator_terminated() {
        let input = "ab,";
        assert_eq!(Ok((",", "ab")), not_separator(input));
    }

    #[test]
    fn not_separator_followed() {
        let input = "ab,cd";
        assert_eq!(Ok((",cd", "ab")), not_separator(input));
    }

    #[test]
    fn escaped_separator_single() {
        let input = ",,";
        assert_eq!(Ok(("", ",,")), escaped_separator(input));
    }
}

use nom::{
    bytes::complete::{is_not, tag},
    IResult,
};

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

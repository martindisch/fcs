use std::{
    fmt::{self, Display},
    num::{IntErrorKind, ParseIntError},
    ops::RangeInclusive,
};

/// The header segment with its offsets.
///
/// Offsets are zero if the segment doesn't exist (only applies to analysis) or
/// if the offsets don't fit in the header and are instead written to the
/// text segment (applies to all).
#[derive(Debug, PartialEq)]
pub struct Header {
    pub version: String,
    pub text_offsets: RangeInclusive<usize>,
    pub data_offsets: RangeInclusive<usize>,
    pub analysis_offsets: RangeInclusive<usize>,
}

impl TryFrom<&str> for Header {
    type Error = ParseIntError;

    fn try_from(header: &str) -> Result<Self, Self::Error> {
        let version = header[0..=5].to_string();
        let text_start = header[10..=17].trim_start().parse::<usize>()?;
        let text_end = header[18..=25].trim_start().parse::<usize>()?;
        let data_start = header[26..=33].trim_start().parse::<usize>()?;
        let data_end = header[34..=41].trim_start().parse::<usize>()?;
        let analysis_start = header[42..=49]
            .trim_start()
            .parse::<usize>()
            .or_else(zero_when_empty)?;
        let analysis_end = header[50..=57]
            .trim_start()
            .parse::<usize>()
            .or_else(zero_when_empty)?;

        Ok(Self {
            version,
            text_offsets: text_start..=text_end,
            data_offsets: data_start..=data_end,
            analysis_offsets: analysis_start..=analysis_end,
        })
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<10}{:>8}{:>8}{:>8}{:>8}{:>8}{:>8}",
            self.version,
            self.text_offsets.start(),
            self.text_offsets.end(),
            self.data_offsets.start(),
            self.data_offsets.end(),
            self.analysis_offsets.start(),
            self.analysis_offsets.end(),
        )
    }
}

/// Replaces errors caused by empty string with value 0, which is semantically
/// identical for the analysis segment.
fn zero_when_empty(error: ParseIntError) -> Result<usize, ParseIntError> {
    if error.kind() == &IntErrorKind::Empty {
        Ok(0)
    } else {
        Err(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header1() {
        let header =
            "FCS3.0         256    1545    1792  202456       0       0";
        let parsed_header = Header {
            version: "FCS3.0".into(),
            text_offsets: 256..=1545,
            data_offsets: 1792..=202456,
            analysis_offsets: 0..=0,
        };

        assert_eq!(parsed_header, Header::try_from(header).unwrap());
    }

    #[test]
    fn header1_spaces() {
        let header =
            "FCS3.0         256    1545    1792  202456                ";
        let parsed_header = Header {
            version: "FCS3.0".into(),
            text_offsets: 256..=1545,
            data_offsets: 1792..=202456,
            analysis_offsets: 0..=0,
        };

        assert_eq!(parsed_header, Header::try_from(header).unwrap());
    }

    #[test]
    fn header2() {
        let header =
            "FCS3.0         256    1545       0       0       0       0";
        let parsed_header = Header {
            version: "FCS3.0".into(),
            text_offsets: 256..=1545,
            data_offsets: 0..=0,
            analysis_offsets: 0..=0,
        };

        assert_eq!(parsed_header, Header::try_from(header).unwrap());
    }

    #[test]
    fn header3() {
        let header =
            "FCS3.0      202451  203140    1792  202450       0       0";
        let parsed_header = Header {
            version: "FCS3.0".into(),
            text_offsets: 202451..=203140,
            data_offsets: 1792..=202450,
            analysis_offsets: 0..=0,
        };

        assert_eq!(parsed_header, Header::try_from(header).unwrap());
    }

    #[test]
    fn write_header1() {
        let header = Header {
            version: "FCS3.0".into(),
            text_offsets: 256..=1545,
            data_offsets: 1792..=202456,
            analysis_offsets: 0..=0,
        };
        let formatted = header.to_string();

        assert_eq!(
            "FCS3.0         256    1545    1792  202456       0       0",
            formatted
        );
    }

    #[test]
    fn write_header2() {
        let header = Header {
            version: "FCS3.0".into(),
            text_offsets: 256..=1545,
            data_offsets: 0..=0,
            analysis_offsets: 0..=0,
        };
        let formatted = header.to_string();

        assert_eq!(
            "FCS3.0         256    1545       0       0       0       0",
            formatted
        );
    }

    #[test]
    fn write_header3() {
        let header = Header {
            version: "FCS3.0".into(),
            text_offsets: 202451..=203140,
            data_offsets: 1792..=202450,
            analysis_offsets: 0..=0,
        };
        let formatted = header.to_string();

        assert_eq!(
            "FCS3.0      202451  203140    1792  202450       0       0",
            formatted
        );
    }
}

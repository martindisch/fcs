use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::{borrow::Borrow, io::Cursor};
use thiserror::Error;

use super::text::Text;

/// The data segment with its events.
#[derive(Debug, PartialEq)]
pub struct Data {
    pub events: Vec<f32>,
}

impl TryFrom<(&Text, &[u8])> for Data {
    type Error = DataError;

    fn try_from((text, data): (&Text, &[u8])) -> Result<Self, Self::Error> {
        let mode = text
            .pairs
            .get("$MODE")
            .map(Borrow::borrow)
            .unwrap_or("undefined");
        if mode != "L" {
            return Err(DataError::UnsupportedMode);
        }

        let data_type = text
            .pairs
            .get("$DATATYPE")
            .map(Borrow::borrow)
            .unwrap_or("undefined");
        if data_type != "F" {
            return Err(DataError::UnsupportedDataType);
        }

        let mut events = vec![0_f32; data.len() / 4];
        let mut reader = Cursor::new(data);
        match text.pairs.get("$BYTEORD").map(Borrow::borrow) {
            Some("1,2,3,4") => {
                reader.read_f32_into::<LittleEndian>(&mut events)
            }
            Some("4,3,2,1") => reader.read_f32_into::<BigEndian>(&mut events),
            _ => return Err(DataError::UnsupportedByteOrder),
        }
        .map_err(|_| DataError::BadRead)?;

        Ok(Self { events })
    }
}

/// The error for parsing the data segment.
#[derive(Debug, PartialEq, Error)]
pub enum DataError {
    #[error("unsupported mode, only list mode is supported")]
    UnsupportedMode,
    #[error("unsupported byte order, only 1,2,3,4 and 4,3,2,1 are supported")]
    UnsupportedByteOrder,
    #[error("unsupported data type, only 32-bit floats are supported")]
    UnsupportedDataType,
    #[error("data segment could not be exactly read into allocated vector")]
    BadRead,
}

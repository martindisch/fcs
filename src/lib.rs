//! A low-level Flow Cytometry Standard (FCS) file serializer/deserializer.
//!
//! ## Features
//!
//! - Parsing header segment
//! - Parsing text segment
//! - Parsing data segment of 32-bit floats in list mode

mod data;
mod header;
mod text;

pub use data::{Data, DataError};
pub use header::Header;
pub use text::{Text, TextError};

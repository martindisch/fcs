//! A low-level Flow Cytometry Standard (FCS) file serializer/deserializer.
//!
//! ## Features
//!
//! - Parsing header segment
//! - Parsing text segment

mod header;
mod text;

pub use header::Header;
pub use text::{Text, TextError};

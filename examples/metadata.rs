use eyre::{Result, WrapErr};
use fcs::{Header, Text};
use std::{env, fs, str};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let contents = fs::read(path).wrap_err("could not read file")?;

    let header = str::from_utf8(&contents[..=57])
        .wrap_err("header is not a valid string")?;
    let header =
        Header::try_from(header).wrap_err("could not parse header")?;
    println!("{header:#?}");

    let text = str::from_utf8(&contents[header.text_offsets])
        .wrap_err("text is not a valid string")?;
    let text =
        Text::try_from(text).wrap_err("could not parse text segment")?;
    println!("{text:#?}");

    Ok(())
}

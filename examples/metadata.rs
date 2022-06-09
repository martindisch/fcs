use eyre::{eyre, Result};
use fcs::{Header, Text};
use std::{env, fs, str};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args
        .get(1)
        .ok_or(eyre!("no argument for input file given"))?;

    let contents = fs::read(path)?;

    let header = str::from_utf8(&contents[..=57])?;
    let header = Header::try_from(header)?;
    println!("{header:#?}");

    let text = str::from_utf8(&contents[header.text_offsets])?;
    let text = Text::try_from(text)?;
    println!("{text:#?}");

    Ok(())
}

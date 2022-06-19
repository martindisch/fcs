use eyre::{eyre, Result};
use fcs::{Data, Header, Text};
use std::{
    env, fs,
    fs::File,
    io::{BufWriter, Write},
    str,
};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let in_path = args
        .get(1)
        .ok_or(eyre!("no argument for input file given"))?;
    let out_path = args
        .get(2)
        .ok_or(eyre!("no argument for output file given"))?;

    let contents = fs::read(in_path)?;

    let header = str::from_utf8(&contents[..=57])?;
    let header = Header::try_from(header)?;

    let text = str::from_utf8(&contents[header.text_offsets.clone()])?;
    let text = Text::try_from(text)?;

    let data =
        Data::try_from((&text, &contents[header.data_offsets.clone()]))?;

    let mut writer = BufWriter::new(File::create(out_path)?);
    write!(writer, "{header}{text}")?;
    data.write(&mut writer)?;

    Ok(())
}

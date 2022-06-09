use eyre::{eyre, Result, WrapErr};
use fcs::{Data, Header, Text};
use std::{
    borrow::Borrow,
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

    let contents = fs::read(in_path).wrap_err("could not read file")?;

    let header = str::from_utf8(&contents[..=57])
        .wrap_err("header is not a valid string")?;
    let header =
        Header::try_from(header).wrap_err("could not parse header")?;

    let text = str::from_utf8(&contents[header.text_offsets])
        .wrap_err("text is not a valid string")?;
    let text =
        Text::try_from(text).wrap_err("could not parse text segment")?;

    let data = Data::try_from((&text, &contents[header.data_offsets]))
        .wrap_err("could not parse data segment")?;

    let parameter_count = text
        .pairs
        .get("$PAR")
        .map(Borrow::borrow)
        .map(str::parse::<usize>)
        .ok_or(eyre!("number of parameters not set"))?
        .wrap_err("could not parse number of parameters")?;

    let mut writer = BufWriter::new(File::create(out_path)?);
    for event in data.events.as_slice().chunks(parameter_count) {
        for number in &event[..event.len() - 1] {
            write!(writer, "{number},")?;
        }
        writeln!(writer, "{}", event[event.len() - 1])?;
    }

    Ok(())
}

use std::error::Error;
use std::{fs::File, io::BufReader};
use std::io::prelude::*;
use crates_index::{Index};

fn main() -> Result<(), Box<dyn Error>> {
    // Read in a list of target words to check
    let in_file = File::open("top1000words.txt")?;
    let buf_reader = BufReader::new(in_file);

    // Check if each word is an existing crate on the crates.io index
    let index = Index::new_cargo_default()?;
    let mut available_names = Vec::<String>::new();
    buf_reader.lines().map(|l| l.unwrap()).for_each(|l| {
        if let None = index.crate_(l.as_str()) {
            available_names.push(l);
        }
    });

    // Output the unclaimed words to a file
    let mut out_file = File::create("availableNames.txt")?;
    for name in available_names {
        write!(out_file, "{}\n", name)?;
    }

    Ok(())
}

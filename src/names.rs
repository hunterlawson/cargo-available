use crates_index::Index;
use std::io::Write;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_names_from_file(path: &String) -> Result<Vec<String>, Box<dyn Error>> {
    // Read in the list of names
    let in_file = File::open(path)?;
    let buf_reader = BufReader::new(in_file);
    Ok(buf_reader.lines().map(|l| l.unwrap()).collect())
}

pub fn get_available_names(names: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
    // Check if each word is an existing crate on the crates.io index
    let index = Index::new_cargo_default()?;
    let mut available_names = Vec::<String>::new();
    for name in names {
        if let None = index.crate_(name.as_str()) {
            available_names.push(name);
        }
    }

    Ok(available_names)
}

pub fn output_names_to_file(path: &String, names: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut out_file = File::create(path)?;
    for name in names {
        write!(out_file, "{}\n", name)?;
    }

    Ok(())
}

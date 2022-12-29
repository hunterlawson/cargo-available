//! Climb functions

use crate::{
    names::{get_available_names, output_names_to_file, read_names_from_file},
    FILE_OUTPUT_OPTION, FILE_QUERY_OPTION,
};
use climb::{FunctionInput, FunctionOption, FunctionOptions, FunctionResult};

pub fn query(input: FunctionInput, options: FunctionOptions) -> FunctionResult {
    // Check the command options
    let mut from_file = false;
    let mut out_file_name = None;
    let query_string = input.get(0).unwrap();

    let _file_query = format!("--{}", FILE_QUERY_OPTION);
    let _file_output = format!("--{}", FILE_OUTPUT_OPTION);

    for option in options {
        match option {
            FunctionOption(_file_query, None) => from_file = true,
            FunctionOption(_file_output, filename) => out_file_name = filename,
        }
    }

    let mut query_names = vec![];

    // Query string is the filename to query from
    if from_file {
        query_names = read_names_from_file(query_string).unwrap();
    } else {
        query_names.push(query_string.clone());
    }

    // Get crate data
    let available_names = match get_available_names(query_names) {
        Ok(names) => names,
        Err(e) => {
            return Err(format!("Error getting crate data from crates.io: {:?}", e));
        }
    };

    // Write output to a file or display on screen
    if available_names.len() == 0 {
        if from_file {
            println!("None of the crate names in the query are available");
        } else {
            println!("`{query_string}` is already taken");
        }
        return Ok(None);
    }

    if let Some(filename) = out_file_name {
        match output_names_to_file(&filename, available_names) {
            Ok(_) => (),
            Err(e) => {
                return Err(format!(
                    "Error writing available names to {filename}: {:?}",
                    e
                ));
            }
        }
    } else {
        for name in available_names {
            println!("`{name}` is available");
        }
    }

    Ok(None)
}

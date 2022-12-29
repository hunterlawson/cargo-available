use climb::*;
use crate_name_finder::functions::query;
use crate_name_finder::{FILE_OUTPUT_OPTION, FILE_QUERY_OPTION};

fn main() {
    let _app_result = create_app!()
        .command(
            Command::new("query", "Query the crates.io index for unused names", query)
                .alias("q")
                .arg("query")
                .option(
                    CommandOption::new(
                        FILE_QUERY_OPTION,
                        "Read in query from a file (interprets <QUERY> as a filename)",
                    )
                    .alias("f"),
                )
                .option(
                    CommandOption::new(FILE_OUTPUT_OPTION, "Output the query result to a file")
                        .alias("o")
                        .arg("file_name"),
                ),
        )
        .run();
}

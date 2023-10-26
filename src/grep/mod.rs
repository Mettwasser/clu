pub mod grep_line;
mod logic;

use crate::args::prelude::*;
use crate::errors::Result;
use console::style;
use logic::{print_lines, search_in_text};
use std::fs;

/// The function reads a file, searches for specified patterns in its contents, and prints the lines
/// that contain the patterns.
///
/// Arguments:
///
/// * `args`: The `args` parameter is a slice of `String` values, which represents the command-line
/// arguments passed to the program.
///
/// Returns:
///
/// The function `run` returns a `Result` type.
pub fn run(args: &[String]) -> Result<()> {
    let args = GrepArguments::parse(args)?;

    let file_contents = fs::read_to_string(args.filename).map_err(|e| {
        format!(
            "{} Couldn't open/read the file: {e}",
            style("[ERROR]").red()
        )
    })?;

    let found_lines = search_in_text(&file_contents, &args.patterns, args.ignore_case);

    print_lines(found_lines);

    Ok(())
}

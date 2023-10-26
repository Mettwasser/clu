use super::grep_line::GrepLine;
use crate::utils::grep::count_digits;
use console::style;

pub fn search_in_text(text: &str, patterns: &[String], ignore_case: bool) -> Vec<GrepLine> {
    text.lines()
        .enumerate()
        .filter_map(|(line_number, line)| {
            let formatted_line = patterns.iter().fold(line.to_string(), |acc, pattern| {
                if ignore_case {
                    acc.replace(
                        &pattern.to_lowercase(),
                        style(&pattern.to_lowercase())
                            .green()
                            .bold()
                            .to_string()
                            .as_str(),
                    )
                    .replace(
                        &pattern.to_uppercase(),
                        style(&pattern.to_uppercase())
                            .green()
                            .bold()
                            .to_string()
                            .as_str(),
                    )
                } else {
                    acc.replace(pattern, style(&pattern).green().bold().to_string().as_str())
                }
            });

            if formatted_line != line {
                Some(GrepLine {
                    line_number: line_number + 1,
                    line: formatted_line,
                })
            } else {
                None
            }
        })
        .collect()
}

/// The function `print_lines` takes a vector of `GrepLine` structs and prints each line along with its
/// line number.
///
/// Arguments:
///
/// * `grep_lines`: The `grep_lines` parameter is a vector of `GrepLine` structs.
pub fn print_lines(grep_lines: Vec<GrepLine>) {
    let line_count_space_buffer = 5;

    for line in grep_lines {
        let line_count_space = count_digits(line_count_space_buffer - line.line_number as i32);

        println!(
            "{}.{}{}",
            style(line.line_number).bold(),
            " ".repeat(line_count_space),
            line.line
        )
    }
}

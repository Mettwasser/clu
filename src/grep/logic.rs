use super::grep_line::GrepLine;
use crate::utils::grep::count_digits;
use console::style;

pub fn search_in_text(
    text: &str,
    patterns: &[String],
    ignore_case: bool,
    color: bool,
) -> Vec<GrepLine> {
    text.lines()
        .enumerate()
        .map(|(line_number, line)| {
            let mut grepline = GrepLine::new(line.to_string(), line_number + 1);

            for pattern in patterns {
                grepline.apply_pattern(pattern, ignore_case, color);
            }

            grepline
        })
        .filter(|grepline| grepline.can_show)
        .collect()
}

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

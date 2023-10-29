use console::style;

#[derive(Debug)]
pub struct GrepLine {
    pub line_number: usize,
    pub line: String,
    pub can_show: bool,
}

impl GrepLine {
    pub fn new(line: String, line_number: usize) -> Self {
        GrepLine {
            line_number,
            line,
            can_show: false,
        }
    }

    pub fn apply_pattern(&mut self, pattern: &str, ignore_case: bool, color: bool) {
        self.can_show = if ignore_case {
            self.line.to_lowercase().contains(&pattern.to_lowercase())
        } else {
            self.line.contains(pattern)
        };

        if self.can_show && color {
            color_line(&mut self.line, pattern, ignore_case);
        }
    }
}
fn color_line(line: &mut String, pattern: &str, ignore_case: bool) {
    let flagged_line = if ignore_case {
        line.to_lowercase()
    } else {
        line.to_string()
    };

    let flagged_pattern = if ignore_case {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    // replace in-place
    for (i, m) in flagged_line.rmatch_indices(&flagged_pattern) {
        line.replace_range(
            i..i + m.len(),
            style(line.get(i..(i + m.len())).unwrap())
                .green()
                .to_string()
                .as_str(),
        );
    }
}

/// The `GrepLine` struct represents a line of text along with its line number.
///
/// Properties:
///
/// * `line_number`: An unsigned integer representing the line number of the line in a file.
/// * `line`: The `line` property is a `String` that represents a line of text in a file.
#[derive(Debug)]
pub struct GrepLine {
    pub line_number: usize,
    pub line: String,
}

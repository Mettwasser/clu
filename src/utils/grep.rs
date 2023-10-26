/// The function `count_digits` takes an integer `n` and returns the number of digits in `n`.
///
/// Arguments:
///
/// * `n`: The parameter `n` is an integer value.
///
/// Returns:
///
/// The function `count_digits` returns the number of digits in the given integer `n`.
pub fn count_digits(n: i32) -> usize {
    (n.checked_ilog10().unwrap_or(0) + 1) as usize
}

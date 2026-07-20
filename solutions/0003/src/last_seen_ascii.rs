/// Uses an ASCII byte as a direct index into a 128-entry position table.
///
/// Returns `None` when `input` contains a non-ASCII character. For ASCII input,
/// time is `O(n)` and auxiliary space is exactly 128 `usize` entries.
#[must_use]
pub fn longest_unique_substring(input: &str) -> Option<usize> {
    if !input.is_ascii() {
        return None;
    }

    // Zero means unseen; otherwise the value is the last byte position + 1.
    let mut last_seen_plus_one = [0_usize; 128];
    let mut left = 0;
    let mut best = 0;

    for (right, &byte) in input.as_bytes().iter().enumerate() {
        let previous_plus_one = last_seen_plus_one[usize::from(byte)];
        left = left.max(previous_plus_one);
        best = best.max(right - left + 1);
        last_seen_plus_one[usize::from(byte)] = right + 1;
    }

    Some(best)
}

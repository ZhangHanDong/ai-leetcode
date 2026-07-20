use std::collections::HashMap;

/// Jumps the left boundary using each character's most recent position.
///
/// Expected time is `O(n)` and auxiliary space is `O(min(n, U))`. Positions
/// are character indices, not UTF-8 byte offsets.
#[must_use]
pub fn longest_unique_substring(input: &str) -> usize {
    let mut last_seen = HashMap::new();
    let mut left = 0;
    let mut best = 0;

    for (right, character) in input.chars().enumerate() {
        if let Some(previous) = last_seen.insert(character, right) {
            left = left.max(previous + 1);
        }
        best = best.max(right - left + 1);
    }

    best
}

use std::collections::HashSet;

/// Enumerates every start position and extends it until the first duplicate.
///
/// Expected time is `O(n * min(n, U))` when hash-table operations are expected
/// `O(1)`; this becomes `O(n^2)` when `U` grows with `n`. The set uses
/// `O(min(n, U))` space, while the current `Vec<char>` buffer makes total
/// auxiliary space `O(n)`.
#[must_use]
pub fn longest_unique_substring(input: &str) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let mut best = 0;

    for start in 0..chars.len() {
        let mut seen = HashSet::new();

        for &character in &chars[start..] {
            if !seen.insert(character) {
                break;
            }
            best = best.max(seen.len());
        }
    }

    best
}

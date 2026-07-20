use std::collections::HashSet;

/// Enumerates every start position and extends it until the first duplicate.
///
/// Time: `O(n^2)` for an unbounded character set. Space: `O(min(n, U))`, where
/// `U` is the number of distinct characters in the input domain.
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

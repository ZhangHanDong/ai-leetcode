use std::collections::HashSet;

/// Maintains the current valid window explicitly in a `HashSet`.
///
/// Each character is inserted once and removed at most once. Expected time is
/// `O(n)`. The set uses `O(min(n, U))` space, while the current `Vec<char>`
/// buffer makes total auxiliary space `O(n)`.
#[must_use]
pub fn longest_unique_substring(input: &str) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let mut window = HashSet::new();
    let mut left = 0;
    let mut best = 0;

    for (right, &character) in chars.iter().enumerate() {
        while window.contains(&character) {
            window.remove(&chars[left]);
            left += 1;
        }

        window.insert(character);
        best = best.max(right - left + 1);
    }

    best
}

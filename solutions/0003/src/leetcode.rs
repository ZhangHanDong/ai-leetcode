/// Adapter matching `LeetCode`'s required method signature.
pub struct Solution;

impl Solution {
    /// Solves `LeetCode` 3 using the Unicode-safe last-seen map.
    ///
    /// # Panics
    ///
    /// Panics only if the answer exceeds `i32::MAX`, which is excluded by the
    /// judge's documented maximum input length of 50,000.
    #[must_use]
    #[allow(clippy::needless_pass_by_value)] // The judge requires `String` by value.
    pub fn length_of_longest_substring(input: String) -> i32 {
        let length = crate::last_seen_hash_map::longest_unique_substring(&input);
        i32::try_from(length).expect("LeetCode 3 limits input length to 50,000")
    }
}

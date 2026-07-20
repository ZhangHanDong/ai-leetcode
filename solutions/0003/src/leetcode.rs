/// Adapter matching `LeetCode`'s required method signature.
pub struct Solution;

impl Solution {
    /// Solves `LeetCode` 3 using the ASCII direct-address table.
    ///
    /// # Panics
    ///
    /// Panics if called with input outside `LeetCode`'s documented ASCII domain.
    #[must_use]
    #[allow(clippy::needless_pass_by_value)] // The judge requires `String` by value.
    pub fn length_of_longest_substring(input: String) -> i32 {
        let length = crate::last_seen_ascii::longest_unique_substring(&input)
            .expect("LeetCode 3 documents an ASCII input domain");
        i32::try_from(length).expect("LeetCode 3 limits input length to 50,000")
    }
}

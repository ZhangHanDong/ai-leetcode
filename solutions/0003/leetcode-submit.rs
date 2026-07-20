impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;

        let mut last_seen = HashMap::new();
        let mut left = 0;
        let mut best = 0;

        for (right, character) in s.chars().enumerate() {
            if let Some(previous) = last_seen.insert(character, right) {
                left = left.max(previous + 1);
            }
            best = best.max(right - left + 1);
        }

        // Safe because the problem limits the input length to 50,000.
        best as i32
    }
}

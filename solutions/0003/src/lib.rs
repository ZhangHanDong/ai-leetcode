//! `LeetCode` 3: Longest Substring Without Repeating Characters.
//!
//! The first three implementations count Unicode scalar values (`char`). The
//! direct-address table is an ASCII specialization and returns `None` for
//! inputs outside that domain.

pub mod brute_force;
pub mod last_seen_ascii;
pub mod last_seen_hash_map;
pub mod leetcode;
pub mod sliding_hash_set;
pub mod trace;

/// Function type shared by the Unicode-capable implementations.
pub type UnicodeSolver = fn(&str) -> usize;

/// All Unicode-capable implementations, used by shared tests.
pub const UNICODE_SOLVERS: &[(&str, UnicodeSolver)] = &[
    ("brute_force", brute_force::longest_unique_substring),
    (
        "sliding_hash_set",
        sliding_hash_set::longest_unique_substring,
    ),
    (
        "last_seen_hash_map",
        last_seen_hash_map::longest_unique_substring,
    ),
];

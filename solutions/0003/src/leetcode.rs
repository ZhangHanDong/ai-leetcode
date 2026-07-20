#![allow(
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]

/// Adapter matching `LeetCode`'s required method signature.
pub struct Solution;

// This is the exact snippet shown in the book and submitted to LeetCode.
include!("../leetcode-submit.rs");

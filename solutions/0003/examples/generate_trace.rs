use std::env;

use lc0003_longest_substring::trace::last_seen_hash_map_trace;

fn main() {
    let input = env::args().nth(1).unwrap_or_else(|| "abba".to_owned());
    let trace = last_seen_hash_map_trace(&input, "duplicate-inside-window");
    println!(
        "{}",
        serde_json::to_string_pretty(&trace).expect("trace is serializable")
    );
}

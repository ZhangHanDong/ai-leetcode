use lc0003_longest_substring::trace::last_seen_hash_map_trace;

#[test]
fn trace_is_deterministic_and_steps_are_contiguous() {
    let first = last_seen_hash_map_trace("abba", "duplicate-inside-window");
    let second = last_seen_hash_map_trace("abba", "duplicate-inside-window");

    assert_eq!(first, second);
    assert_eq!(first.final_state["result"], 2);
    assert!(
        first
            .events
            .iter()
            .enumerate()
            .all(|(step, event)| event.step == step)
    );
    assert!(
        first
            .events
            .iter()
            .any(|event| event.event_type == "window_shrink")
    );
}

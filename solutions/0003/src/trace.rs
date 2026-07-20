use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// A renderer-independent execution trace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraceDocument {
    pub schema_version: u8,
    pub problem_id: String,
    pub solution_id: String,
    pub case_id: String,
    pub initial_state: Value,
    pub events: Vec<TraceEvent>,
    pub final_state: Value,
}

/// One semantic state transition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraceEvent {
    pub step: usize,
    #[serde(rename = "type")]
    pub event_type: String,
    pub payload: Value,
    pub explanation: String,
}

/// Runs the last-seen `HashMap` implementation while recording semantic events.
#[must_use]
pub fn last_seen_hash_map_trace(input: &str, case_id: &str) -> TraceDocument {
    let mut last_seen = HashMap::new();
    let mut left = 0;
    let mut best = 0;
    let mut events = Vec::new();

    for (right, character) in input.chars().enumerate() {
        push_event(
            &mut events,
            "visit",
            json!({ "index": right, "character": character.to_string() }),
            format!("读取位置 {right} 的字符 {character:?}"),
        );

        if let Some(previous) = last_seen.insert(character, right) {
            let next_left = left.max(previous + 1);
            if next_left != left {
                push_event(
                    &mut events,
                    "window_shrink",
                    json!({
                        "from_left": left,
                        "to_left": next_left,
                        "duplicate": character.to_string(),
                        "previous_index": previous
                    }),
                    format!("字符 {character:?} 在窗口内重复，左边界跳到 {next_left}"),
                );
                left = next_left;
            }
        }

        push_event(
            &mut events,
            "window_expand",
            json!({ "left": left, "right": right, "length": right - left + 1 }),
            format!("合法窗口更新为 [{left}, {})", right + 1),
        );

        let window_length = right - left + 1;
        if window_length > best {
            best = window_length;
            push_event(
                &mut events,
                "best_update",
                json!({ "best": best, "left": left, "right": right }),
                format!("当前合法窗口更长，最优长度更新为 {best}"),
            );
        }
    }

    TraceDocument {
        schema_version: 1,
        problem_id: "lc-0003".to_owned(),
        solution_id: "last-seen-hash-map".to_owned(),
        case_id: case_id.to_owned(),
        initial_state: json!({ "input": input, "left": 0, "best": 0 }),
        events,
        final_state: json!({ "result": best }),
    }
}

fn push_event(events: &mut Vec<TraceEvent>, event_type: &str, payload: Value, explanation: String) {
    events.push(TraceEvent {
        step: events.len(),
        event_type: event_type.to_owned(),
        payload,
        explanation,
    });
}

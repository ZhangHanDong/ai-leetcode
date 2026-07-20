use lc0003_longest_substring::{UNICODE_SOLVERS, last_seen_ascii};

#[test]
fn all_implementations_share_ascii_examples() {
    let cases = [
        ("", 0),
        ("a", 1),
        ("aaaa", 1),
        ("tigertrail", 5),
        ("abba", 2),
        ("dvdf", 3),
        ("a b!a", 4),
    ];

    for (input, expected) in cases {
        for &(name, solver) in UNICODE_SOLVERS {
            assert_eq!(solver(input), expected, "solver={name}, input={input:?}");
        }
        assert_eq!(
            last_seen_ascii::longest_unique_substring(input),
            Some(expected),
            "solver=last_seen_ascii, input={input:?}"
        );
    }
}

#[test]
fn unicode_implementations_count_chars_not_bytes() {
    let cases = [("你好吗你", 3), ("🦀a🦀b", 3), ("éèêé", 3)];

    for (input, expected) in cases {
        for &(name, solver) in UNICODE_SOLVERS {
            assert_eq!(solver(input), expected, "solver={name}, input={input:?}");
        }
        assert_eq!(last_seen_ascii::longest_unique_substring(input), None);
    }
}

#[test]
fn optimized_implementations_match_brute_force_exhaustively() {
    const ALPHABET: &[u8] = b"abcd";

    for length in 0..=7_u32 {
        let count = ALPHABET.len().pow(length);
        for encoded in 0..count {
            let input = decode_ascii_word(encoded, length, ALPHABET);
            let expected = lc0003_longest_substring::brute_force::longest_unique_substring(&input);

            for &(name, solver) in &UNICODE_SOLVERS[1..] {
                assert_eq!(solver(&input), expected, "solver={name}, input={input:?}");
            }
            assert_eq!(
                last_seen_ascii::longest_unique_substring(&input),
                Some(expected),
                "solver=last_seen_ascii, input={input:?}"
            );
        }
    }
}

fn decode_ascii_word(mut encoded: usize, length: u32, alphabet: &[u8]) -> String {
    let mut bytes = Vec::with_capacity(usize::try_from(length).expect("small test length"));
    for _ in 0..length {
        bytes.push(alphabet[encoded % alphabet.len()]);
        encoded /= alphabet.len();
    }
    String::from_utf8(bytes).expect("alphabet is ASCII")
}

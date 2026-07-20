use std::hint::black_box;
use std::time::Duration;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use lc0003_longest_substring::{
    brute_force, last_seen_ascii, last_seen_hash_map, sliding_hash_set,
};

fn compare_ascii(c: &mut Criterion) {
    let mut group = c.benchmark_group("lc0003/ascii-cyclic");
    group
        .sample_size(30)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(2));

    for length in [64, 4_096, 50_000] {
        let input = cyclic_printable_ascii(length);
        group.bench_with_input(BenchmarkId::new("brute-force", length), &input, |b, s| {
            b.iter(|| brute_force::longest_unique_substring(black_box(s)));
        });
        group.bench_with_input(
            BenchmarkId::new("sliding-hash-set", length),
            &input,
            |b, s| b.iter(|| sliding_hash_set::longest_unique_substring(black_box(s))),
        );
        group.bench_with_input(
            BenchmarkId::new("last-seen-hash-map", length),
            &input,
            |b, s| b.iter(|| last_seen_hash_map::longest_unique_substring(black_box(s))),
        );
        group.bench_with_input(
            BenchmarkId::new("last-seen-ascii", length),
            &input,
            |b, s| b.iter(|| last_seen_ascii::longest_unique_substring(black_box(s))),
        );
    }
    group.finish();
}

fn compare_repeat_heavy(c: &mut Criterion) {
    let input = "a".repeat(50_000);
    let mut group = c.benchmark_group("lc0003/repeat-heavy/50000");
    group
        .sample_size(30)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(2));

    group.bench_function("brute-force", |b| {
        b.iter(|| brute_force::longest_unique_substring(black_box(&input)));
    });
    group.bench_function("sliding-hash-set", |b| {
        b.iter(|| sliding_hash_set::longest_unique_substring(black_box(&input)));
    });
    group.bench_function("last-seen-hash-map", |b| {
        b.iter(|| last_seen_hash_map::longest_unique_substring(black_box(&input)));
    });
    group.bench_function("last-seen-ascii", |b| {
        b.iter(|| last_seen_ascii::longest_unique_substring(black_box(&input)));
    });
    group.finish();
}

fn compare_unicode(c: &mut Criterion) {
    let mut group = c.benchmark_group("lc0003/unicode-unique");
    group
        .sample_size(30)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(2));

    for length in [64, 256] {
        let input = unique_unicode(length);
        group.bench_with_input(BenchmarkId::new("brute-force", length), &input, |b, s| {
            b.iter(|| brute_force::longest_unique_substring(black_box(s)));
        });
        group.bench_with_input(
            BenchmarkId::new("sliding-hash-set", length),
            &input,
            |b, s| b.iter(|| sliding_hash_set::longest_unique_substring(black_box(s))),
        );
        group.bench_with_input(
            BenchmarkId::new("last-seen-hash-map", length),
            &input,
            |b, s| b.iter(|| last_seen_hash_map::longest_unique_substring(black_box(s))),
        );
    }
    group.finish();
}

fn cyclic_printable_ascii(length: usize) -> String {
    const ALPHABET: &[u8] =
        b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[]{};:,.<>?/|~` ";
    let bytes = (0..length)
        .map(|index| ALPHABET[index % ALPHABET.len()])
        .collect();
    String::from_utf8(bytes).expect("benchmark alphabet is ASCII")
}

fn unique_unicode(length: u32) -> String {
    (0..length)
        .map(|offset| char::from_u32(0x1_000 + offset).expect("benchmark range is valid Unicode"))
        .collect()
}

criterion_group!(
    benches,
    compare_ascii,
    compare_repeat_heavy,
    compare_unicode
);
criterion_main!(benches);

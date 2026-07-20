# Repository Instructions

## Mission

Build an evidence-backed algorithm book from structured knowledge, executable solutions, deterministic traces, and reproducible benchmarks.

## Source-of-truth boundaries

- `content/`: problem, pattern, data-structure, and relationship knowledge.
- `solutions/`: executable source code and test fixtures.
- `artifacts/`: machine-produced traces and benchmark reports.
- `book/src/generated/`: disposable renderer output; never edit it by hand.
- `book/src/`: hand-written reader onboarding and book-level navigation only.

## Required workflow

Every new problem moves through these states in order:

`draft -> tested -> verified -> benchmarked -> visualized -> editorial-reviewed -> published`

An agent must not advance a state unless the corresponding acceptance criteria in `specs/` pass.

## Non-negotiable rules

1. Do not scrape or reproduce complete LeetCode problem statements.
2. Never claim that one solution is faster without a committed benchmark report and environment metadata.
3. Every complexity claim must include its assumptions and derivation.
4. Every published solution must compile or execute against all checked-in cases.
5. Every animation event must map to an explicit algorithm state transition.
6. Prefer stable IDs over display names in cross-references.
7. Keep generated files out of manual reviews unless the task is specifically about rendering.
8. AI-authored prose must distinguish verified facts, analysis, and editorial judgment.

## Writing rules

- Problem pages explain one problem deeply; pattern pages synthesize at least three problems.
- Start from the invariant and state representation, not from memorized code.
- Compare approaches by constraints, complexity, memory, implementation risk, and measured results.
- Avoid repeating the same proof or template across chapters; link to the canonical pattern page.
- Use the templates in `templates/` and record the page budget before drafting long chapters.

## Code conventions

- Keep platform adapters separate from the core algorithm.
- Tests should call the core algorithm directly.
- Trace instrumentation must be optional and must not change algorithm semantics.
- Benchmarks run optimized builds and use deterministic, documented datasets.


.PHONY: book serve test check bench-lc3 trace-lc3 clean

book:
	mdbook build book

serve:
	mdbook serve book --open

test:
	cargo test --workspace
	mdbook test book

check:
	cargo fmt --all -- --check
	cargo test --workspace
	cargo clippy --workspace --all-targets -- -D warnings
	mdbook test book
	mdbook build book
	node --check book/theme/algo-lab.js
	node --check book/theme/algo-viz.js

bench-lc3:
	cargo bench -p lc0003-longest-substring --bench compare -- --noplot

trace-lc3:
	cargo run -q -p lc0003-longest-substring --example generate_trace -- abba > artifacts/traces/lc-0003/last-seen-hash-map/abba.json

clean:
	mdbook clean book

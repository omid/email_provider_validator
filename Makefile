check:
	@cargo +nightly fmt
	@cargo clippy --fix --allow-dirty --allow-staged --all-targets -- -D warnings -A clippy::extra_unused_lifetimes
	@cargo update --dry-run
	@cargo +nightly udeps --all-targets
	@cargo machete
	@cargo outdated -wR

check_nightly:
	@cargo +nightly clippy --fix --allow-dirty --allow-staged  --all-features --all-targets

check_strictly:
	@cargo +nightly clippy --fix --allow-dirty --allow-staged --all-features --all-targets -- -W clippy::all -W clippy::pedantic -W clippy::cargo

test:
	@cargo test --features free,disposable

doc:
	@DOCS_RS=1 cargo doc --open --no-deps --all-features

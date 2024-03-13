check:
	@cargo +nightly fmt
	@cargo clippy --fix --allow-dirty --allow-staged --all-targets -- -D warnings -A clippy::extra_unused_lifetimes
	@cargo update --dry-run
	@cargo +nightly udeps --all-targets
	@cargo machete
	@cargo outdated -wR

check_nightly:
	@cargo +nightly clippy --fix --allow-dirty --allow-staged

check_strictly:
	@cargo +nightly clippy --fix --allow-dirty --allow-staged --all-features --all-targets -- -W clippy::all -W clippy::pedantic -W clippy::cargo -A clippy::missing_errors_doc -A clippy::extra_unused_lifetimes -A clippy::cast_sign_loss -A clippy::cast_possible_truncation -A clippy::missing-panics-doc -A clippy::module_name_repetitions -A clippy::cast_precision_loss -A clippy::cast_possible_wrap -A clippy::used_underscore_binding -A clippy::multiple_crate_versions -A clippy::option_option -A let_underscore_drop

check_very_strictly:
	@cargo +nightly clippy --fix --allow-dirty --allow-staged --all-features --all-targets -- -W clippy::all -W clippy::pedantic -W clippy::cargo -A clippy::module-name-repetitions -A clippy::missing-panics-doc -A clippy::missing-errors-doc -A clippy::option-option

test:
	@cargo test --features free,disposable

VERBOSE := $(if ${CI},--verbose,)
CARGO := cargo

test:
	${CARGO} test ${VERBOSE} --all -- --nocapture

check-fmt:
	cargo +nightly fmt ${VERBOSE} --all -- --check

fmt:
	cargo +nightly fmt ${VERBOSE} --all

clippy:
	${CARGO} clippy ${VERBOSE} --all --all-targets --all-features -- \
		-D warnings -D clippy::enum_glob_use -D clippy::clone_on_ref_ptr

ci: fmt check-fmt clippy test

run-quizzer-dev:
	RUST_BACKTRACE=full RUST_LOG=info,hyper=info,quizzer=trace${RUST_LOG} ${CARGO} run --bin quizzer-server
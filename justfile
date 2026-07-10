default:
    @just --list

# Format code
fmt:
    cargo fmt --all
    cargo sort --workspace --grouped

# Run the test suite
test:
    cargo nextest run
    cargo nextest run --all-features

# Run all CI checks
ci: fmt test
    cargo check
    cargo doc --no-deps
    cargo fmt --check
    cargo clippy -- -D warnings

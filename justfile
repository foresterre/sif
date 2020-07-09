clippy:
    cargo clippy --all-targets --all-features --all -- -D warnings

test:
    cargo test --all

before-push:
    cargo fmt --all
    cargo clippy --all-targets --all-features --all -- -D warnings
    cargo test --all
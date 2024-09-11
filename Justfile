default:
    just --list

dev:
    RUST_LOG=trace bacon clippy

run *args:
    cargo run -- {{args}}

run-release *args:
    cargo run --release -- {{args}}

alias d := dev
alias r := run
alias rr := run-release

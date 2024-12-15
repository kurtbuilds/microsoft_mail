set dotenv-load
set positional-arguments
set export

run *ARGS:
    cargo run -- "$@"

test *ARGS:
    cargo test -- "$@"

build:
    cargo build

install:
    cargo install --path . --locked

example NAME:
    cargo run --example $NAME
alias e := example

check:
    cargo check

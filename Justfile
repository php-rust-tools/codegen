default:
  @just --list

# build the library
build:
    cargo build

# regenerate schema
update-examples:
    rm examples/simple.php examples/complete.php
    cargo run --example simple >> examples/simple.php
    cargo run --example complete >> examples/complete.php

# detect linting problems.
lint:
    cargo fmt --all -- --check
    cargo clippy

# fix linting problems.
fix:
    cargo fmt
    cargo clippy --fix --allow-dirty --allow-staged

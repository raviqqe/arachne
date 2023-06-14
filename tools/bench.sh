#!/bin/sh

set -e

cargo build --release
cargo install hyperfine

bench/fibonacci/main.sh

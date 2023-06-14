#!/bin/sh

set -e

cargo build --release
cargo install hyperfine

for directory in fibonacci sum; do
  bench/$directory/main.sh
done

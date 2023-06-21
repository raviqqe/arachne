#!/bin/sh

set -e

cd $(dirname $0)/..

cargo build --release
cargo install hyperfine

for directory in fibonacci sum tak; do
  directory=bench/$directory

  if which petite >/dev/null; then
    scheme="petite --script $directory/main.scm"
  fi

  hyperfine --sort command -w 2 \
    "target/release/arachne < $directory/main.arc" \
    "python3 $directory/main.py" \
    $scheme
done

#!/bin/sh

set -e

run() {
  hyperfine --sort command -w 2 "$@"
}

cd $(dirname $0)/..

cargo build --release
cargo install hyperfine

for name in fibonacci sum tak; do
  directory=bench/$name

  echo '>>>' $name

  run \
    -n target/release/arachne '<' $directory/main.arc \
    -n target/release/arachne-old '<' $directory/main.arc \
    -n python3 $directory/main.py

  if which petite >/dev/null && [ -r $directory/main.scm ]; then
    run "petite --script $directory/main.scm"
  fi
done

#!/bin/sh

set -e

cd $(dirname $0)/..

rnm --no-git --include '.*\.rs$' -r '#\[inline[^\]]*\]' ''
cargo fmt --all

#!/bin/sh

set -e

cd $(dirname $0)/..

rnm -r '#\[inline[^\]]*\]' ''
cargo fmt --all

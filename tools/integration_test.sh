#!/bin/sh

set -e

cd $(dirname $0)/..

bundler install

export ROOT=$PWD
export PATH=$PWD/target/release:$PATH

cargo build --release
cucumber --publish-quiet "$@"

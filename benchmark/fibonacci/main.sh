#!/bin/sh

set -e

cd $(dirname $0)

cargo build --release

hyperfine '../target/release/arachne fibonacci.arc' 'python3 ./fibonacci.py'

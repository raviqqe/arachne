#!/bin/sh

set -e

cd $(dirname $0)

hyperfine '../target/release/arachne fibonacci.arc' 'python3 ./fibonacci.py'

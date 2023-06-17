#!/bin/sh

set -e

cd $(dirname $0)

hyperfine -w 2 '../../target/release/arachne < main.arc' 'python3 main.py'

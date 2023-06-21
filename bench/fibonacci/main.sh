#!/bin/sh

set -e

cd $(dirname $0)

hyperfine -w 1 --sort command '../../target/release/arachne < main.arc' 'python3 main.py' 'petite --script main.scm'

#!/bin/sh

set -e

cd $(dirname $0)

hyperfine '../../target/release/arachne < main.arc' 'python3 main.py' 'csi -s main.scm'

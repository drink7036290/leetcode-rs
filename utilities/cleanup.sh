#!/usr/bin/env bash

set -e  # Exit on error
cargo clean
find . -type f \( -name "*.profraw" -o -name "lcov*.info" \) -exec rm {} \;
rm -f metrics.json
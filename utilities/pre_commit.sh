#!/usr/bin/env bash

set -e  # Exit on error

rustfmt --edition 2024 $(git ls-files '*.rs')

cargo clippy --all --tests --all-features --no-deps

markdownlint '**/*.md' --ignore node_modules
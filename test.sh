#!/usr/bin/env bash
set -euo pipefail

./target/release/subs --mappings-file mappings.txt test.txt -

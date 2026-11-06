#!/bin/bash

# Set "chatgpt.cliExecutable": "/Users/<USERNAME>/code/bgs/scripts/debug-bgs.sh" in VSCode settings to always get the 
# latest bgs-rs binary when debugging Bgs Extension.


set -euo pipefail

BGS_RS_DIR=$(realpath "$(dirname "$0")/../bgs-rs")
(cd "$BGS_RS_DIR" && cargo run --quiet --bin bgs -- "$@")
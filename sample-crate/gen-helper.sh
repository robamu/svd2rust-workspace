#!/bin/sh
# Requires the file to parse as input

# Use installed tool by default
svd2rust_prefix="cargo run --bin svd2rust -- -i"
cmd="${svd2rust_prefix} $1"
echo "Executing command ${cmd}"
eval ${cmd}
rm -rf src
form -i lib.rs -o src/ && rm lib.rs

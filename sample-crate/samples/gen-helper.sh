#!/bin/sh
# Requires the file to parse as input

# Use installed tool by default
svd2rust_prefix="cargo run -p svd2rust -- -i"
cmd="${svd2rust_prefix} $1"
echo "Executing command ${cmd}"
eval ${cmd}
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
rm -rf ../src
mv src/ ..
mv device.x ..
mv build.rs ..
cargo fmt -p sample-crate

#!/bin/sh
# Requires the file to parse as input
if [ $# -eq 0 ]; then
	echo "Please pass the name of the SVD file to parse as an argument"
	exit 1
fi

# Use installed tool by default
svd2rust_prefix="cargo run -p svd2rust -- -i"
cmd="${svd2rust_prefix} $1"
echo "Executing command ${cmd}"
eval ${cmd}

echo "Applying form on lib.rs"
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
rm -rf ../src
mv src/ ..
mv device.x ..
mv build.rs ..

echo "Applying cargo fmt"
cargo fmt -p sample-crate

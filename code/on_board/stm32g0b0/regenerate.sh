#!/bin/sh
if [ ! -f "./STM32G0B0.svd" ]
then
    echo "Wrong path, please move to directory containing STM32G0B0.svd." >&2
    exit 1
fi

# remove current generated code
rm -rf "src"

# generate fresh code
svd2rust -i "./STM32G0B0.svd" -o . --reexport-interrupt

# beautify
form -i "./lib.rs" -o "src"

# remove lib.rs intermediate artifact
rm "./lib.rs"

# beautify harder
cargo fmt

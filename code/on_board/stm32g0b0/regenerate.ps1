If (-not (Test-Path -Path ".\STM32G0B0.svd")) {
    Throw "Wrong path, please move to directory containing STM32G0B0.svd."
}

# remove current generated code
Remove-Item -Recurse -Force -Path "src"

# generate fresh code
svd2rust -i ".\STM32G0B0.svd" -o . --reexport-interrupt

# beautify
form -i ".\lib.rs" -o "src"

# remove lib.rs intermediate artifact
Remove-Item -Path ".\lib.rs"

# beautify harder
cargo fmt

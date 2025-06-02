#!/usr/bin/env bash
cargo run -- --input tests/input/rojo.json --output tests/output/json.luau
cargo run -- --input tests/input/hello.txt --output tests/output/txt.luau
cargo run -- --input tests/input/pseudo-enum.toml --output tests/output/toml.luau
cargo run -- --input tests/input/midas.yaml --output tests/output/yaml.luau
cargo run -- --input tests/input/vehicle.csv --output tests/output/csv.luau
cargo run -- --input tests/input/vehicle.tsv --output tests/output/tsv.luau
cargo run -- --input tests/input/infrastruct.xlsx --output tests/output/page-xlsx.luau --page City
cargo run -- --input tests/input/infrastruct.xlsx --output tests/output/id-xlsx.luau --key Id
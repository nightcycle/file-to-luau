#!/usr/bin/env bash
cargo run -- --input tests/input/rojo.json --out tests/output/json.luau
cargo run -- --input tests/input/hello.txt --out tests/output/txt.luau
cargo run -- --input tests/input/pseudo-enum.toml --out tests/output/toml.luau
cargo run -- --input tests/input/midas.yaml --out tests/output/yaml.luau
cargo run -- --input tests/input/vehicle.csv --out tests/output/csv.luau
cargo run -- --input tests/input/vehicle.tsv --out tests/output/tsv.luau
cargo run -- --input tests/input/infrastruct.xlsx --out tests/output/xlsx.luau --page City
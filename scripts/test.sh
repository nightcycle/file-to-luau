#!/usr/bin/env bash
cargo run -- --input tests/input/rojo.json --out tests/output/json.luau
cargo run -- --input tests/input/hello.txt --out tests/output/txt.luau
cargo run -- --input tests/input/pseudo-enum.toml --out tests/output/toml.luau
cargo run -- --input tests/input/midas.yaml --out tests/output/yaml.luau
#!/bin/sh
set -eu

BIN="rust-starter-bin"

$BIN --help
# assert matches version in Cargo.toml

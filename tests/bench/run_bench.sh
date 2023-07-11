#!/bin/sh
set -eu

ROOT="$(pwd)"

export TEMPDIR=$(mktemp -d)
(
    cd "${TEMPDIR}"
    hyperfine \
        --warmup 1 \
        --max-runs 10 \
        "$ROOT/tests/bench/bench_basic.sh"
)
rm -rf "${TEMPDIR}"

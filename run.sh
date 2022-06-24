#!/bin/bash

function compare() {
    bench_name="$1"
    version_cmd="$2"
    bench_cmd="$3"

    echo "============ Version ============"
    echo "${bench_name}:"
    eval "$version_cmd"
    echo "============ Results ============"
    eval "$bench_cmd"
    echo "================================="

    echo
}

compare \
    "deno_bindgen + Deno" \
    "deno --version" \
    "deno run --unstable --allow-env --allow-read --allow-write --allow-ffi --allow-hrtime ./deno_bindgen/bench.js"

compare \
    "NAPI and Node" \
    "node --version" \
    "node napi/bench.js"

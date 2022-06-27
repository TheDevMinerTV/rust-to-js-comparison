#!/bin/bash

function build() {
    name="$1"
    cmd="$2"

    echo "==== Building ${name} binding ===="
    eval "$cmd"
    echo "==============================="

    echo
}

# Note that we're not going into the `deno_bindgen` folder,
# When using Cargo workspaces we can't build these bindings in the workspace folder
# and so we have to build them from the root folder.
# We also have to nuke Deno's cache (see https://github.com/denoland/deno_bindgen/issues/57)
build "Deno" "deno_bindgen --release && rm -rf ~/.cache/deno/plug"

cd napi
build "NAPI" "yarn napi build --platform --release"
cd ..

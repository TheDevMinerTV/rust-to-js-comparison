#!/bin/bash

rm -rf bindings
rm -rf ~/.cache/deno/plug
rm -rf napi/*.node

cargo clean

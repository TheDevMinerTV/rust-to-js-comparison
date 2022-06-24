#!/bin/bash

deno install -Afq -n deno_bindgen https://deno.land/x/deno_bindgen/cli.ts

npm i -g yarn

cd napi
yarn
cd ..

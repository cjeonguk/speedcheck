#!/bin/bash

for ((i = 0; i < 15; i++))
do
  # node wasm-js/main.js
  # node js/main.js
  ./rust/target/release/rust
done
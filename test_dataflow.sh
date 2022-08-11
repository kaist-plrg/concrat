#! /bin/bash

set -e

cargo build --release

for d in dataflow_examples/*; do
  if [ -d "$d" ]; then
    cargo run --release --bin dataflow -- -d deps -i $d -t
  fi
done

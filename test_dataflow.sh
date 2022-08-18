#! /bin/bash

set -e

cargo build --release

for d in dataflow_examples/*; do
  if [ -d "$d" ]; then
    cargo run --release --bin dataflow -- -d deps_crate/target/debug/deps -i $d -t
  fi
done

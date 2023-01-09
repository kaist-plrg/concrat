#! /bin/bash

set -e

cargo build --release

for d in examples/*; do
  if [ -d "$d" ]; then
    td=`mktemp -d`
    ./scripts/test_dir.sh $d $td
    rm -rf $td
  fi
done

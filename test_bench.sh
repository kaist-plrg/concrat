#! /bin/bash

set -e

cargo build --release

# for d in examples/*; do
while read d; do
  if [ -d "$d" ]; then
    td=`mktemp -d`
    ./test_dir.sh $d $td
    rm -rf $td
  fi
done < test_list

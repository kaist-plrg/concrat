#! /bin/bash

set -e

cargo build --release

while read d; do
  if [ -d "$d" ]; then
    td=`mktemp -d`
    ! ./scripts/test_dir.sh $d $td
    rm -rf $td
  fi
done < scripts/list_fail

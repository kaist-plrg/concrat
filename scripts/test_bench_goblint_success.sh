#! /bin/bash

set -e

cargo build --release

while read d; do
  if [ -d "$d" ] && [ -f "$d/a.xml" ]; then
    td=`mktemp -d`
    GOBLINT=yes ./scripts/test_dir.sh $d $td
    rm -rf $td
  fi
done < scripts/list_goblint_success

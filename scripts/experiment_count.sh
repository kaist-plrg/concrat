#! /bin/bash

echo -e 'C\tRust\tMutex\tRwlock\tSpin\tCond\tProject'
while read d; do
  if [[ $d != *"-fixed" ]]; then
    C=`cloc --csv $d/main.c | tail -n 1 | tr ',' '\n' | tail -n 1`
    RUST=`wc -l $d/main.rs | tr ' ' '\n' | head -n 1`
    API=`api_counter $d/main.rs`
    echo $C $RUST $API $(basename $d) | tr ' ' '\t'
  fi
done < scripts/list_all

#! /bin/bash

for d in ./*; do
  if [ -d "$d" ]; then
    echo "$d"
    cd "$d"
    make clean
    make
    cd ..
  fi
done

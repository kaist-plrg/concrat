#! /bin/bash

function show(){
  case "$1" in
    "") echo "." ;;
    "0") echo "SUCC" ;;
    *) echo "FAIL" ;;
  esac
}

echo -e 'C\tRust\tTrans\tProject'
while read d; do
  X=
  Y=
  Z=
  cd $d
  make test C=yes > /dev/null 2>&1
  X=$?
  if [ $X -eq 0 ]; then
    make test > /dev/null 2>&1
    Y=$?
    if [ $Y -eq 0 ]; then
      make transformation > /dev/null 2>&1
      make test > /dev/null 2>&1
      Z=$?
      make revert > /dev/null 2>&1
    fi
  fi
  cd ../..
  echo $(show $X) $(show $Y) $(show $Z) $(basename $d) | tr ' ' '\t'
done < scripts/list_correctness

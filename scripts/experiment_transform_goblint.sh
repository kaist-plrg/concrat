#! /bin/bash

echo -e 'Succ\tProject'
while read d; do
  if [ -f "$d/a.xml" ]; then
    td=`mktemp -d`
    GOBLINT=yes ./scripts/test_dir.sh $d $td > /dev/null 2>&1
    case "$?" in
      0) SUC="yes" ;;
      *) SUC="no" ;;
    esac
    rm -rf $td
    echo $SUC $(basename $d) | tr ' ' '\t'
  fi
done < scripts/list_all

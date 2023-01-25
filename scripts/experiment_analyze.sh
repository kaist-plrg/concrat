#! /bin/bash

echo -e 'Time\tProject'
while read d; do
  cd $d
  rm -f a.xml
  START=`date +%s%N`
  make a.xml > /dev/null 2>&1
  END=`date +%s%N`
  NSEC=`expr $END - $START`
  MSEC=`expr $NSEC / 1000000`
  TIME=`echo $MSEC 1000 | awk '{print $1 / $2}'`
  echo $TIME $(basename $d) | tr ' ' '\t'
  cd ../..
done < scripts/list_goblint

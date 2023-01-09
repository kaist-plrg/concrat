#! /bin/bash

echo -e 'Ana\tTra\tIns\tDel\tSucc\tProject'
while read d; do
  td=`mktemp -d`
  RES=`./scripts/test_dir.sh $d $td 2> /dev/null`
  case "$?" in
    0) SUC="yes" ;;
    *) SUC="no" ;;
  esac
  rm -rf $td
  ANA=`echo $RES | tr ' ' '\n' | sed -n 3p`
  TRA=`echo $RES | tr ' ' '\n' | sed -n 6p`
  RES=`echo $RES | tr ' ' '\n' | tail -n 6`
  INS=`echo $RES | tr ' ' '\n' | sed -n 1p`
  DEL=`echo $RES | tr ' ' '\n' | sed -n 3p`
  echo $ANA $TRA $INS $DEL $SUC $d | tr ' ' '\t'
done < scripts/list_all

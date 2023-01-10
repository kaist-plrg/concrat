#! /bin/bash

set -e

X=100000
V=`./main $X 1000`
if [ "$V" != "$X" ]; then
  exit -1
fi

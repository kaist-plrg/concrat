#! /bin/bash

if [ "$1" = "" ]; then
  exit 1
fi

if [ -d "$1" ]; then
  exit 1
fi

mkdir $1
ln -s ../Makefile.txt $1/Makefile
ln -s ../b.c $1/b.c
touch $1/a.c

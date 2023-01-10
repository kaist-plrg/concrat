#! /bin/bash

set -e

LD_PRELOAD="./libfaketime.so.1" FAKETIME="@2005-03-29 14:14:14" timeout 5s ./main

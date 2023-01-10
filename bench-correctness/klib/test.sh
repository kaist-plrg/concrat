#! /bin/bash

set -e

./main main.c 8 8 > result_
diff result result_
rm result_

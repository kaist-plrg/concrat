#! /bin/bash

set -e

./main | tail -n 1 > result_
diff result result_
rm result_

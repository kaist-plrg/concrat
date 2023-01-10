#! /bin/bash

set -e

./main --ifile modes1.bin > result_
diff result result_
rm result_

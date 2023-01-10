#! /bin/bash

set -e

./main 02hello.strm > result_
./main 03fizzbuzz.strm >> result_
./main 04emit.strm >> result_
./main 05emit.strm >> result_
./main 05filter.strm >> result_
./main 09match.strm >> result_
diff result result_
rm result_

#! /bin/bash

set -e

./main -kf pigz.c ; ./main -t pigz.c.gz
./main -kfb 32 pigz.c ; ./main -t pigz.c.gz
./main -kfp 1 pigz.c ; ./main -t pigz.c.gz
./main -kfz pigz.c ; ./main -t pigz.c.zz
./main -kfK pigz.c ; ./main -t pigz.c.zip
printf "" | ./main -cdf | wc -c | test `cat` -eq 0
printf "x" | ./main -cdf | wc -c | test `cat` -eq 1
printf "xy" | ./main -cdf | wc -c | test `cat` -eq 2
printf "xyz" | ./main -cdf | wc -c | test `cat` -eq 3
(printf "w" | gzip ; printf "x") | ./main -cdf | wc -c | test `cat` -eq 2
(printf "w" | gzip ; printf "xy") | ./main -cdf | wc -c | test `cat` -eq 3
(printf "w" | gzip ; printf "xyz") | ./main -cdf | wc -c | test `cat` -eq 4
rm -f pigz.c.gz pigz.c.zz pigz.c.zip

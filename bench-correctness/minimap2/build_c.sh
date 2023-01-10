#! /bin/bash

set -e

gcc -o main main.c ./ksw2_extd2_sse.c ./ksw2_exts2_sse.c ksw2_extz2_sse.c ksw2_ll_sse.c -pthread -lm -lz

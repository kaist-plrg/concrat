#! /bin/bash

set -e

gcc -o main main.c -pthread -lfuse3 -lglib-2.0

#! /bin/bash

set -e

gcc -o main main.c acosw.S -pthread

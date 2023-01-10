#! /bin/bash

set -e

gcc -o main main.c -pthread -lrt

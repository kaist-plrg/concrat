#! /bin/bash

set -e

gcc -o main main.c -pthread -lpcre -llzma -lz

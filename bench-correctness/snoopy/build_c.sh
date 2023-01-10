#! /bin/bash

set -e

gcc -o main main.c libsnoopy-test-cli.a -pthread -ldl

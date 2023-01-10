#! /bin/bash

set -e

gcc -rdynamic -o main main.c *.a -pthread -lm -ldl -laio -lmysqlclient

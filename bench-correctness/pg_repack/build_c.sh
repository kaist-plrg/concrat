#! /bin/bash

set -e

gcc -o main main.c -pthread -L/usr/lib/postgresql/13/lib -lpq -lpgcommon -lpgport

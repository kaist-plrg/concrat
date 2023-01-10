#! /bin/bash

set -e

cd tests && timeout 100s ./test_run.sh

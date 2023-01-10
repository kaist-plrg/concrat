#! /bin/bash

set -e

ln -f -s `pwd`/main /usr/bin/pg_repack
/bin/bash -c 'cd regress && ./travis_test.sh'

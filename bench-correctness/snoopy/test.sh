#! /bin/bash

set -e

cd tests/cli
/bin/bash -c "!(./test.sh | grep '^FAIL')"
cd ../..

cd tests/combined
/bin/bash -c "!(./test.sh | grep '^FAIL')"
cd ../..

cd tests/configfile
/bin/bash -c "!(./test.sh | grep '^FAIL')"
cd ../..

cd tests/datasource
/bin/bash -c "!(./test.sh | grep '^FAIL')"
cd ../..

cd tests/filter
/bin/bash -c "!(./test.sh | grep '^FAIL')"
cd ../..

cd tests/general
/bin/bash -c "!(./test.sh | grep '^FAIL')"
cd ../..

cd tests/message
/bin/bash -c "!(./test.sh | grep '^FAIL')"
cd ../..

cd tests/output
/bin/bash -c "!(./test.sh | grep '^FAIL')"
cd ../..

cd tests/threads
/bin/bash -c "!(./test.sh | grep '^FAIL')"
cd ../..

cd tests/unit
/bin/bash -c "!(./test.sh | grep '^FAIL')"
cd ../..

rm tests/*/*.log tests/*/*.trs

#! /bin/bash

/bin/bash ../../build/aux/test-driver --test-name general-symbol-leaks.sh --log-file general-symbol-leaks.sh.log --trs-file general-symbol-leaks.sh.trs --color-tests no --enable-hard-errors yes --expect-failure no -- ./general-symbol-leaks.sh

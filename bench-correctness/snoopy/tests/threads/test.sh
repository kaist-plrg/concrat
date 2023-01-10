#! /bin/bash

/bin/bash ../../build/aux/test-driver --test-name threads_creation.sh --log-file threads_creation.sh.log --trs-file threads_creation.sh.trs --color-tests no --enable-hard-errors yes --expect-failure no -- ./threads_creation.sh
/bin/bash ../../build/aux/test-driver --test-name threads_fork_exec.sh --log-file threads_fork_exec.sh.log --trs-file threads_fork_exec.sh.trs --color-tests no --enable-hard-errors yes --expect-failure no -- ./threads_fork_exec.sh
/bin/bash ../../build/aux/test-driver --test-name threads_try-to-segfault.sh --log-file threads_try-to-segfault.sh.log --trs-file threads_try-to-segfault.sh.trs --color-tests no --enable-hard-errors yes --expect-failure no -- ./threads_try-to-segfault.sh

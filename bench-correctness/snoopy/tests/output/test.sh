#! /bin/bash

/bin/bash ../../build/aux/test-driver --test-name output_devnull.sh --log-file output_devnull.sh.log --trs-file output_devnull.sh.trs --color-tests no --enable-hard-errors yes --expect-failure no -- ./output_devnull.sh
/bin/bash ../../build/aux/test-driver --test-name output_file.sh --log-file output_file.sh.log --trs-file output_file.sh.trs --color-tests no --enable-hard-errors yes --expect-failure no -- ./output_file.sh
/bin/bash ../../build/aux/test-driver --test-name output_noop.sh --log-file output_noop.sh.log --trs-file output_noop.sh.trs --color-tests no --enable-hard-errors yes --expect-failure no -- ./output_noop.sh
/bin/bash ../../build/aux/test-driver --test-name output_stderr.sh --log-file output_stderr.sh.log --trs-file output_stderr.sh.trs --color-tests no --enable-hard-errors yes --expect-failure no -- ./output_stderr.sh
/bin/bash ../../build/aux/test-driver --test-name output_stdout.sh --log-file output_stdout.sh.log --trs-file output_stdout.sh.trs --color-tests no --enable-hard-errors yes --expect-failure no -- ./output_stdout.sh

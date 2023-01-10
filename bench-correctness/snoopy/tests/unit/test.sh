#! /bin/bash

/bin/bash ../../build/aux/test-driver --test-name unit-action-log-message-dispatch.sh --log-file unit-action-log-message-dispatch.sh.log --trs-file unit-action-log-message-dispatch.sh.trs --color-tests no --enable-hard-errors yes --expect-failure no -- ./unit-action-log-message-dispatch.sh
/bin/bash ../../build/aux/test-driver --test-name unit-action-log-syscall-exec.sh --log-file unit-action-log-syscall-exec.sh.log --trs-file unit-action-log-syscall-exec.sh.trs --color-tests no --enable-hard-errors yes --expect-failure no -- ./unit-action-log-syscall-exec.sh
/bin/bash ../../build/aux/test-driver --test-name unit-error.sh --log-file unit-error.sh.log --trs-file unit-error.sh.trs --color-tests no --enable-hard-errors yes --expect-failure no -- ./unit-error.sh
/bin/bash ../../build/aux/test-driver --test-name unit-ext-ini.sh --log-file unit-ext-ini.sh.log --trs-file unit-ext-ini.sh.trs --color-tests no --enable-hard-errors yes --expect-failure no -- ./unit-ext-ini.sh
/bin/bash ../../build/aux/test-driver --test-name unit-filterregistry.sh --log-file unit-filterregistry.sh.log --trs-file unit-filterregistry.sh.trs --color-tests no --enable-hard-errors yes --expect-failure no -- ./unit-filterregistry.sh
/bin/bash ../../build/aux/test-driver --test-name unit-outputregistry.sh --log-file unit-outputregistry.sh.log --trs-file unit-outputregistry.sh.trs --color-tests no --enable-hard-errors yes --expect-failure no -- ./unit-outputregistry.sh
/bin/bash ../../build/aux/test-driver --test-name unit-util-syslog.sh --log-file unit-util-syslog.sh.log --trs-file unit-util-syslog.sh.trs --color-tests no --enable-hard-errors yes --expect-failure no -- ./unit-util-syslog.sh
/bin/bash ../../build/aux/test-driver --test-name unit-util-systemd.sh --log-file unit-util-systemd.sh.log --trs-file unit-util-systemd.sh.trs --color-tests no --enable-hard-errors yes --expect-failure no -- ./unit-util-systemd.sh

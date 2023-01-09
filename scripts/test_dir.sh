#! /bin/bash

set -e

from=$1
to=$2

shift
shift

if [ "$from" = "" ]; then
  exit 1
fi

if [ "$to" = "" ]; then
  exit 1
fi

if [ ! -d "$from" ]; then
  exit 1
fi

mkdir -p $to
rm -f $to/{*.rs,Cargo.toml,rust-toolchain,a.xml,cfg.dot,lines}
cp -r $from/{*.rs,Cargo.toml,rust-toolchain} $to

if [ "$GOBLINT" = "yes" ]; then
  cp -r $from/{a.xml,cfg.dot,lines} $to
  CMD=goblint2json
  MSG=Summarizing
else
  CMD=dataflow
  MSG=Analyzing
fi

cp $to/main.rs $to/main_old.rs

echo $MSG $from
cargo run --release --bin $CMD -- -i $to -d deps_crate/target/debug/deps $@

echo Translating $from
cargo run --release --bin concrat -- -i $to -d deps_crate/target/debug/deps $@

if [ -x "$(command -v diffstat)" ]; then
  diff -u $to/main_old.rs $to/main.rs | diffstat
fi

cargo fmt -- $to/main.rs $to/main_old.rs

echo Compiling $from
nightly=`cat $to/rust-toolchain`
RUSTFLAGS=-Awarnings cargo +$nightly build --manifest-path $to/Cargo.toml

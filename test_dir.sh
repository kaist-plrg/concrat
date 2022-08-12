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
cp -r $from/{*.rs,Cargo.toml,rust-toolchain} $to

echo Analyzing $from
cargo run --release --bin dataflow -- -i $to -d deps $@

echo Translating $from
cargo run --release --bin concrat -- -i $to -d deps $@

echo Compiling $from
nightly=`cat $to/rust-toolchain`
RUSTFLAGS=-Awarnings cargo +$nightly build --manifest-path $to/Cargo.toml

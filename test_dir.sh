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
cp -r $from/{*.rs,*.xml,Cargo.toml,rust-toolchain} $to
echo 'parking_lot = "0.11.2"' >> $to/Cargo.toml

echo Translating $from
cargo run --release -- -i $to $@

echo Compiling $from
nightly=`cat $to/rust-toolchain`
RUSTFLAGS=-Awarnings cargo +$nightly build --manifest-path $to/Cargo.toml

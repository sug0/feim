#!/bin/sh

set -e

trap 'cleanup' EXIT INT

intermediate=/tmp/_intermediate_img
final=/tmp/_final_img

cleanup() {
    rm -f $intermediate
    rm -f $final
}

cargo build --release --bin toff
cargo build --release --bin blur

n=`expr ${N:-1} + 0`
img=${1:-'-'}

cat $img | ./target/release/toff > $final

for i in $(seq 1 $n); do
    cat $final | ./target/release/blur > $intermediate
    rm $final
    mv $intermediate $final
done

cat $final

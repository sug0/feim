#!/bin/sh

set -e

cargo build --release --bin toff
cargo build --release --bin blur

< "$1" ./target/release/toff | ./target/release/blur | feh -

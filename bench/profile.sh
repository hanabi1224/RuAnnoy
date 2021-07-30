#!/bin/bash

pushd rust
cargo build --release --all-features
valgrind --tool=callgrind -- ./target/release/bencher 200 20000 200 50
gprof2dot -f callgrind callgrind.out.10284 | dot -Tsvg -o perf.svg
popd

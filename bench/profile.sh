#!/bin/bash

pushd rust
cargo build --release --all-features
valgrind --log-fd=0 --log-file=perf.callgrind --tool=callgrind -- ./target/release/bencher 50 10000 200 50
gprof2dot -f callgrind callgrind.out.10284 | dot -Tsvg -o perf.svg
popd

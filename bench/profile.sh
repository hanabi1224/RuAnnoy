#!/bin/bash

# python3 -m pip install -U wheel annoy

# python3 builder.py 256 10000

python3 bencher.py 256 10000 200 1000
valgrind --tool=callgrind --callgrind-out-file=callgrind.nosimd.out -- python3 bencher.py 256 10000 200 10
gprof2dot -f callgrind callgrind.nosimd.out | dot -Tsvg -o perf.py3.svg

pushd rust

cargo build --release
./target/release/bencher 256 10000 200 1000
valgrind --tool=callgrind --callgrind-out-file=callgrind.nosimd.out -- ./target/release/bencher 256 10000 200 10
gprof2dot -f callgrind callgrind.nosimd.out | dot -Tsvg -o ../perf.nosimd.svg

cargo +nightly build --release --all-features
./target/release/bencher 256 10000 200 1000
valgrind --tool=callgrind --callgrind-out-file=callgrind.simd.out -- ./target/release/bencher 256 10000 200 10
gprof2dot -f callgrind callgrind.simd.out | dot -Tsvg -o ../perf.simd.svg

popd

#!/bin/bash

python3 bencher.py 50 10000 200 1000

pushd rust
cargo run --release -- 50 10000 200 1000
popd

pushd java
gradle run --args="50 10000 200 1000"
popd

dotnet run -p dotnet -c Release -- --dim 50 --size 10000 --n-result 200 --n-loop 1000

python3 -m pip install -U wheel annoy

python3 builder.py 128 10000

python3 bencher.py 128 10000 200 1000

pushd rust
cargo run -q --release -- 128 10000 200 1000
cargo +nightly run -q --release --all-features -- 128 10000 200 1000
popd

pushd dart
dart pub get && dart run main.dart 128 10000 200 1000
popd

dotnet run -p dotnet -c Release -- --dim 128 --size 10000 --n-result 200 --n-loop 1000

pushd java
gradlew run --args="128 10000 200 1000"
popd

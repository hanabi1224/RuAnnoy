python3 -m pip install -U wheel annoy
python3 bencher.py 50 10000 200 1000

pushd rust
cargo run --release --all-features -- 50 10000 200 1000
popd

pushd dart
dart pub get && dart run main.dart 50 10000 200 1000
popd

dotnet run -p dotnet -c Release -- --dim 50 --size 10000 --n-result 200 --n-loop 1000

pushd java
gradlew run --args="50 10000 200 1000"
popd

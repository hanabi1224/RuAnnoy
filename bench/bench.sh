#!/bin/bash

export RUSTFLAGS="-Ctarget-feature=+avx"
cat ./bench.bat | sed 's/gradlew/.\/gradlew/' | bash

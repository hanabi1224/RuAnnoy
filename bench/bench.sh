#!/bin/bash

export RUSTFLAGS="-Ctarget-cpu=native"
cat ./bench.bat | sed 's/gradlew/.\/gradlew/' | bash

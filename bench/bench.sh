#!/bin/bash

cat ./bench.bat | sed 's/gradlew/.\/gradlew/' | bash

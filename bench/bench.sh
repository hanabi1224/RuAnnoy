#!/bin/bash

cat ./bench.bat | sed 's/gradlew/sudo .\/gradlew/' | bash

#!/bin/bash
set -euo pipefail
shopt -s inherit_errexit

git clone --depth 1 https://github.com/openjdk/jdk.git
cd jdk
git checkout jdk-20+4

#javac --patch-module java.base=java_rt/rt_src -d java_rt/rt_build java_rt/rt_src/**/*.java

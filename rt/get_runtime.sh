#!/bin/bash
set -euo pipefail
shopt -s inherit_errexit

mkdir -p ./jmods/java.base

podman run --rm -it --userns keep-id -v "./jmods/java.base:/tmp/out" --security-opt label=disable eclipse-temurin:11 sh -c "jmod extract --dir /tmp/out /opt/java/openjdk/jmods/java.base.jmod"
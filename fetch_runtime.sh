#!/bin/bash
set -euo pipefail
shopt -s inherit_errexit

mkdir -p ./rt/jmods/java.base

podman run --rm -it --userns keep-id \
    --security-opt label=disable \
    -v "./rt/jmods/java.base:/tmp/out" \
    eclipse-temurin:11 \
    sh -c "jmod extract --dir /tmp/out /opt/java/openjdk/jmods/java.base.jmod"
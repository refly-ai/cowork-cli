#!/usr/bin/env bash
set -euo pipefail

docker build \
  -f "meta/docker/Dockerfile" \
  -t "cowork-coldstart:hello" \
  --build-arg OPENCODE_PROFILE_FILE="auth.helloworld.json" \
  .

docker run --rm "cowork-coldstart:hello" sh -lc "opencode --version && opencode auth list"

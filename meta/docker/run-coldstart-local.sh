#!/usr/bin/env bash
set -euo pipefail

AUTH_FILE="meta/docker/auth/auth.local.json"

if [[ ! -f "$AUTH_FILE" ]]; then
  cp "meta/docker/auth/auth.local.example.json" "$AUTH_FILE"
  echo "Created $AUTH_FILE from template. Fill in real credentials, then rerun."
  exit 1
fi

docker build \
  -f "meta/docker/Dockerfile" \
  -t "cowork-coldstart:local" \
  --build-arg OPENCODE_PROFILE_FILE="auth.local.json" \
  .

docker run --rm "cowork-coldstart:local" sh -lc "opencode --version && opencode auth list"

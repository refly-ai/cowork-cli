FROM debian:bookworm-slim

ARG OPENCODE_VERSION=v1.2.16
ARG OPENCODE_PROFILE_FILE=auth.helloworld.json

RUN apt-get update \
  && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    fzf \
    git \
    python3 \
    python3-pip \
    ripgrep \
    tmux \
    tar \
    xz-utils \
  && rm -rf /var/lib/apt/lists/*

RUN set -eux; \
  arch="$(uname -m)"; \
  case "$arch" in \
    x86_64) asset="opencode-linux-x64.tar.gz" ;; \
    aarch64|arm64) asset="opencode-linux-arm64.tar.gz" ;; \
    *) echo "unsupported arch: $arch" >&2; exit 1 ;; \
  esac; \
  url="https://github.com/anomalyco/opencode/releases/download/${OPENCODE_VERSION}/${asset}"; \
  tmpdir="$(mktemp -d)"; \
  curl -fsSL -o "$tmpdir/opencode.tar.gz" "$url"; \
  tar -xzf "$tmpdir/opencode.tar.gz" -C "$tmpdir"; \
  install -m 0755 "$tmpdir/opencode" /usr/local/bin/opencode; \
  rm -rf "$tmpdir"

RUN mkdir -p /root/.local/share/opencode
COPY meta/docker/auth/${OPENCODE_PROFILE_FILE} /root/.local/share/opencode/auth.json
RUN chmod 600 /root/.local/share/opencode/auth.json

WORKDIR /workspace

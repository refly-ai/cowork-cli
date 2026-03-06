#!/usr/bin/env bash
set -euo pipefail

REPO="powerformer/cowork-cli"
INSTALL_ROOT="${HOME}/.cowork"
BIN_DIR="${INSTALL_ROOT}/bin"
BIN_PATH="${BIN_DIR}/cowork"
LOCAL_BIN_DIR="${HOME}/.local/bin"
LINK_PATH="${LOCAL_BIN_DIR}/cowork"

usage() {
  cat <<'EOF'
Usage: install.sh [--version cowork-vX.Y.Z|vX.Y.Z|X.Y.Z]

Install strategy:
  - binary: ~/.cowork/bin/cowork
  - symlink: ~/.local/bin/cowork -> ~/.cowork/bin/cowork
EOF
}

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "missing required command: $1" >&2
    exit 1
  fi
}

detect_target() {
  local os arch
  os="$(uname -s)"
  arch="$(uname -m)"
  case "${os}:${arch}" in
    Linux:x86_64) echo "x86_64-unknown-linux-gnu" ;;
    Darwin:x86_64) echo "x86_64-apple-darwin" ;;
    Darwin:arm64|Darwin:aarch64) echo "aarch64-apple-darwin" ;;
    *)
      echo "unsupported platform: ${os}/${arch}" >&2
      exit 1
      ;;
  esac
}

fetch_latest_tag() {
  curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
    | sed -nE 's/.*"tag_name":[[:space:]]*"([^"]+)".*/\1/p' \
    | head -n1
}

normalize_version() {
  local raw="$1"
  if [[ "${raw}" == cowork-v* ]]; then
    printf '%s\n' "${raw}"
  elif [[ "${raw}" == v* ]]; then
    printf 'cowork-%s\n' "${raw}"
  else
    printf 'cowork-v%s\n' "${raw}"
  fi
}

read_checksum() {
  local checksums_file="$1"
  local asset_name="$2"
  awk -v f="${asset_name}" '$2==f || $2=="*"f {print $1; exit}' "${checksums_file}"
}

download_and_verify_asset() {
  local asset_name="$1"
  local dest_dir="$2"
  local base_url="$3"
  local checksums_file="$4"

  local expected
  expected="$(read_checksum "${checksums_file}" "${asset_name}")"
  if [[ -z "${expected}" ]]; then
    echo "checksum not found for ${asset_name}" >&2
    exit 1
  fi

  curl -fsSL -o "${dest_dir}/${asset_name}" "${base_url}/${asset_name}"
  verify_checksum "${dest_dir}/${asset_name}" "${expected}"
}

install_from_asset() {
  local asset_name="$1"
  local src_dir="$2"

  tar -xzf "${src_dir}/${asset_name}" -C "${src_dir}"
  if [[ ! -f "${src_dir}/cowork" ]]; then
    echo "release archive missing cowork binary" >&2
    exit 1
  fi

  install -m 0755 "${src_dir}/cowork" "${BIN_PATH}"
  ln -snf "${BIN_PATH}" "${LINK_PATH}"
  rm -f "${src_dir}/cowork"
}

run_post_install_check() {
  local output
  if ! output="$(${BIN_PATH} --version 2>&1)"; then
    echo "post-install check failed: ${output}" >&2
    return 1
  fi
  return 0
}

verify_checksum() {
  local file="$1"
  local expected="$2"
  local actual=""
  if command -v sha256sum >/dev/null 2>&1; then
    actual="$(sha256sum "${file}" | awk '{print $1}')"
  else
    actual="$(shasum -a 256 "${file}" | awk '{print $1}')"
  fi
  if [[ "${actual}" != "${expected}" ]]; then
    echo "checksum mismatch: expected ${expected}, got ${actual}" >&2
    exit 1
  fi
}

VERSION=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --version)
      VERSION="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

require_cmd curl
require_cmd tar
require_cmd mktemp

TARGET="$(detect_target)"
if [[ -z "${VERSION}" ]]; then
  VERSION="$(fetch_latest_tag)"
  if [[ -z "${VERSION}" ]]; then
    echo "failed to resolve latest release tag" >&2
    exit 1
  fi
else
  VERSION="$(normalize_version "${VERSION}")"
fi

ASSET="cowork-${VERSION}-${TARGET}.tar.gz"
BASE_URL="https://github.com/${REPO}/releases/download/${VERSION}"
ASSET_URL="${BASE_URL}/${ASSET}"
CHECKSUMS_URL="${BASE_URL}/checksums.txt"

tmp_dir="$(mktemp -d)"
trap 'rm -rf "${tmp_dir}"' EXIT

echo "Downloading ${ASSET_URL}"
curl -fsSL -o "${tmp_dir}/checksums.txt" "${CHECKSUMS_URL}"
download_and_verify_asset "${ASSET}" "${tmp_dir}" "${BASE_URL}" "${tmp_dir}/checksums.txt"

mkdir -p "${BIN_DIR}" "${LOCAL_BIN_DIR}"
install_from_asset "${ASSET}" "${tmp_dir}"

if ! run_post_install_check; then
  if [[ "$(uname -s):$(uname -m)" == "Linux:x86_64" ]]; then
    fallback_target="x86_64-unknown-linux-musl"
    fallback_asset="cowork-${VERSION}-${fallback_target}.tar.gz"
    echo "Trying fallback asset: ${fallback_asset}"
    download_and_verify_asset "${fallback_asset}" "${tmp_dir}" "${BASE_URL}" "${tmp_dir}/checksums.txt"
    install_from_asset "${fallback_asset}" "${tmp_dir}"
    run_post_install_check || {
      echo "fallback asset failed to run; check runtime compatibility" >&2
      exit 1
    }
  else
    echo "installed binary failed runtime check" >&2
    exit 1
  fi
fi

echo "Installed cowork ${VERSION} to ${BIN_PATH}"
echo "Linked ${LINK_PATH} -> ${BIN_PATH}"
if [[ ":${PATH}:" != *":${LOCAL_BIN_DIR}:"* ]]; then
  echo "Note: ${LOCAL_BIN_DIR} is not in PATH."
  echo "Add it manually to your shell profile when convenient."
fi

#!/usr/bin/env bash

set -u

REPO_URL_DEFAULT="git@github.com:powerformer/consensus.git"
REPO_URL="${COWORK_CLONE_REPO_URL:-$REPO_URL_DEFAULT}"
INSTALL_URL="https://raw.githubusercontent.com/powerformer/cowork-cli/main/install.sh"
COWORK_HOME_DEFAULT="${COWORK_HOME:-${HOME}/.cowork}"
COWORK_BIN_DEFAULT="${COWORK_HOME_DEFAULT}/bin/cowork"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --repo-url)
      REPO_URL="${2:-}"
      shift 2
      ;;
    -h|--help)
      cat <<'EOF'
Usage: bash bootstrap.sh [--repo-url <git-url>]

Checks the local cold-start prerequisites for cowork-managed knowledge repositories.
This script does not mutate auth or ssh state.
EOF
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      exit 1
      ;;
  esac
done

PASS_COUNT=0
FAIL_COUNT=0
NEED_COWORK=0
NEED_NODE=0
NEED_GH=0
NEED_SSH=0
NEED_REPO=0

line() {
  printf '%s\n' "$1"
}

section() {
  printf '\n%s\n' "$1"
}

pass() {
  PASS_COUNT=$((PASS_COUNT + 1))
  printf '[pass] %s\n' "$1"
}

fail() {
  FAIL_COUNT=$((FAIL_COUNT + 1))
  printf '[fail] %s\n' "$1"
}

info() {
  printf '       %s\n' "$1"
}

check_cmd() {
  local name="$1"
  if command -v "$name" >/dev/null 2>&1; then
    pass "command available: $name"
    info "path=$(command -v "$name")"
    return 0
  fi
  fail "command missing: $name"
  return 1
}

run_capture() {
  local tmp rc
  tmp="$(mktemp)"
  if "$@" >"$tmp" 2>&1; then
    rc=0
  else
    rc=$?
  fi
  cat "$tmp"
  rm -f "$tmp"
  return "$rc"
}

first_line() {
  printf '%s' "$1" | python3 - <<'PY'
import sys
text = sys.stdin.read().strip().splitlines()
print(text[0] if text else "")
PY
}

section "Bootstrap Check"
line "repo_url: ${REPO_URL}"
line "install_url: ${INSTALL_URL}"
line "cowork_home: ${COWORK_HOME_DEFAULT}"
line "cowork_bin: ${COWORK_BIN_DEFAULT}"

section "Commands"
check_cmd bash
check_cmd curl
check_cmd git
HAS_GH=0
if check_cmd gh; then HAS_GH=1; fi
check_cmd ssh
HAS_NODE=0
if check_cmd node; then HAS_NODE=1; fi
HAS_NPM=0
if check_cmd npm; then HAS_NPM=1; fi
HAS_OPENCODE=0
if check_cmd opencode; then HAS_OPENCODE=1; fi
HAS_COWORK=0
if check_cmd cowork; then HAS_COWORK=1; fi
if [[ "$HAS_COWORK" -eq 0 ]]; then NEED_COWORK=1; fi

if [[ "$HAS_NODE" -eq 1 ]]; then
  NODE_VERSION_RAW="$(node --version 2>/dev/null || true)"
  NODE_MAJOR="$(printf '%s' "$NODE_VERSION_RAW" | sed -E 's/^v([0-9]+).*/\1/')"
  if [[ "$NODE_MAJOR" == "24" ]]; then
    pass "node major version is 24"
  else
    fail "node major version is not 24"
    NEED_NODE=1
  fi
fi
if [[ "$HAS_NODE" -eq 0 || "$HAS_NPM" -eq 0 ]]; then
  NEED_NODE=1
fi

section "Versions"
if [[ "$HAS_NODE" -eq 1 ]]; then
  info "node_version=${NODE_VERSION_RAW:-$(node --version 2>/dev/null || true)}"
fi
if [[ "$HAS_NPM" -eq 1 ]]; then
  info "npm_version=$(npm --version 2>/dev/null || true)"
fi
if [[ "$HAS_OPENCODE" -eq 1 ]]; then
  info "opencode_version=$(opencode --version 2>/dev/null || true)"
fi
if [[ "$HAS_COWORK" -eq 1 ]]; then
  info "cowork_version=$(cowork --version 2>/dev/null || true)"
else
  info "cowork_install_hint=curl -fsSL ${INSTALL_URL} | bash"
  info "cowork_expected_bin=${COWORK_BIN_DEFAULT}"
fi

section "GitHub CLI"
if [[ "$HAS_GH" -eq 1 ]]; then
  GH_OUTPUT="$(run_capture gh auth status)"
  GH_RC=$?
  if [[ "$GH_RC" -eq 0 ]]; then
    pass "gh auth status"
  else
    fail "gh auth status"
    NEED_GH=1
  fi
  info "exit=${GH_RC}"
  [[ "$GH_RC" -ne 0 ]] && info "detail=$(first_line "$GH_OUTPUT")"
fi

section "SSH"
SSH_OUTPUT="$(run_capture ssh -T -o BatchMode=yes -o StrictHostKeyChecking=accept-new git@github.com)"
SSH_RC=$?
if [[ "$SSH_RC" -eq 1 || "$SSH_RC" -eq 0 ]]; then
  pass "ssh handshake to github.com"
else
  fail "ssh handshake to github.com"
  NEED_SSH=1
fi
info "exit=${SSH_RC}"
if [[ "$SSH_RC" -ne 0 && "$SSH_RC" -ne 1 ]]; then
  info "detail=$(first_line "$SSH_OUTPUT")"
fi

section "Repository Access"
GIT_SSH_COMMAND='ssh -o BatchMode=yes -o StrictHostKeyChecking=accept-new'
export GIT_SSH_COMMAND
REPO_OUTPUT="$(run_capture git ls-remote "$REPO_URL" HEAD)"
REPO_RC=$?
unset GIT_SSH_COMMAND
if [[ "$REPO_RC" -eq 0 ]]; then
  pass "git ls-remote ${REPO_URL}"
else
  fail "git ls-remote ${REPO_URL}"
  NEED_REPO=1
fi
info "exit=${REPO_RC}"
if [[ "$REPO_RC" -ne 0 ]]; then
  info "detail=$(first_line "$REPO_OUTPUT")"
fi

section "Summary"
line "pass_count=${PASS_COUNT}"
line "fail_count=${FAIL_COUNT}"

if [[ "$FAIL_COUNT" -eq 0 ]]; then
  line "result=ready"
  exit 0
fi

section "Next Actions"
line "[For Agent] If result is action-needed, do not continue repository initialization. Only restate the actions below or ask for teammate help."
if [[ "$NEED_COWORK" -eq 1 ]]; then
  line "[For User] Install cowork: curl -fsSL ${INSTALL_URL} | bash"
  line "[For Agent] Expected cowork binary path after install: ${COWORK_BIN_DEFAULT}"
fi
if [[ "$NEED_NODE" -eq 1 ]]; then
  line "[For User] Install Node.js 24 and ensure both node and npm are available in PATH."
fi
if [[ "$NEED_GH" -eq 1 ]]; then
  line "[For User] Authenticate GitHub CLI: gh auth login"
fi
if [[ "$NEED_SSH" -eq 1 ]]; then
  line "[For User] Configure GitHub SSH access for this machine; if you do not know how, ask a teammate for help."
fi
if [[ "$NEED_REPO" -eq 1 ]]; then
  line "[For User] Confirm repository access to ${REPO_URL}; if gh/ssh are already configured but access still fails, ask a teammate for help."
fi

line "result=action-needed"
exit 1

#!/usr/bin/env bash
set -euo pipefail

cat <<EOF
Cold-start Task V1

Repository URL: ${CONSENSUS_REPO_URL:-git@github.com:powerformer/consensus.git}
Install script: ${COWORK_INSTALL_URL:-https://raw.githubusercontent.com/powerformer/cowork-cli/main/install.sh}
Cowork beta version: 0.1.4-beta
Cowork home: ${COWORK_HOME:-/workspace/.cowork}
Cowork repo URL: ${COWORK_CLONE_REPO_URL:-git@github.com:powerformer/consensus.git}
Cowork repo alias: ${COWORK_CLONE_REPO_ALIAS:-consensus}

Task prompt file:
  /workspace/meta/docker/TASK_V1.md

Suggested first steps:
1. Install cowork from the install script URL with `--version 0.1.4-beta`.
2. Use cowork to understand repository boundary and local path.
3. Make one minimal documentation-only improvement.
4. Create a draft PR.
EOF

exec bash

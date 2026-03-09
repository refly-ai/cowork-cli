#!/usr/bin/env bash
set -euo pipefail

prompt="$(cat /workspace/meta/docker/TASK_V1.md)

## Runtime Context

- Knowledge repository URL: ${CONSENSUS_REPO_URL}
- cowork install script URL: ${COWORK_INSTALL_URL}
- cowork beta version: 0.1.4-beta
- cowork home: ${COWORK_HOME}
- cowork clone repo URL: ${COWORK_CLONE_REPO_URL}
- cowork clone repo alias: ${COWORK_CLONE_REPO_ALIAS}
"
exec opencode run --model openai/gpt-5.4 "$prompt"

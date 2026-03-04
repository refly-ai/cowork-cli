# cowork

Minimal Rust CLI for cowork clone workflows.

## Environment

- `COWORK_HOME` (optional, default `~/.cowork`)
- `COWORK_CLONE_REPO_URL` (required by `cowork clone init`)
- `COWORK_CLONE_REPO_ALIAS` (optional, default `default`)
- `COWORK_CLONE_REPO_SUBDIR` (optional, default empty)
- `COWORK_CLONE_SESSION` (optional, default `cowork-preview`)
- `COWORK_CLONE_PREVIEW_CMD` (optional, default `npm run dev`)
- `COWORK_CLONE_METADATA_TREE_DEPTH` (optional, default `3`)
- `COWORK_SELF_UPDATE_INSTALL_URL` (optional)

Default install URL:

`https://raw.githubusercontent.com/refly-ai/cowork-cli/main/install.sh`

## Commands

```bash
cowork self-update [--version x.y.z]
cowork clone version
cowork clone init
cowork clone update
cowork clone metadata
cowork clone preview
cowork clone contribute
```

Cold-start contribution path: use branch + PR, then squash merge.

# AGENTS

## 仓库概述

`cowork-cli` 是一个最小 Rust CLI 项目，目标是提供 `cowork` 的冷启动命令闭环：`self-update` 与 `clone` 子命令组（version/init/update/metadata/preview/contribute/resource）。

## 目录规范

- `src/bin/`: CLI 入口（参数与子命令分发）。
- `src/commands/`: 命令实现。
- `src/config.rs`: `COWORK_*` 环境变量与默认值。
- `tests/`: 命令级 smoke tests。
- `.github/workflows/`: CI 与 release 工作流。
- `install.sh`: 发布资产安装脚本。

## 常用命令

- `cargo test`
- `cargo run -- --help`
- `cargo run -- clone --help`
- `cargo run -- clone metadata`
- `cargo run -- clone preview`

## 常见问题

- **`clone init` 失败**：检查 `COWORK_CLONE_REPO_URL` 是否已设置。
- **`clone update/metadata/preview` 失败**：检查目标路径 `COWORK_HOME/clones/COWORK_CLONE_REPO_ALIAS[/COWORK_CLONE_REPO_SUBDIR]` 是否存在。
- **`clone preview` 提示 session 已存在**：执行 `tmux attach -t <session>` 或手动清理旧 session。
- **`self-update` 失败**：检查 `install.sh` 地址可访问，以及当前平台是否有对应 release 资产。

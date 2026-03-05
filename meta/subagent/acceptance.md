# Subagent 验收标准

用于定义本仓库 cold-start 自检的通过条件，确保结论一致可复现。

## 验收维度

1. 环境可用性：容器可构建、可启动、可执行核心命令。
2. 命令闭环：目标命令按预期执行并返回有效结果。
3. 数据隔离：不使用本地目录挂载，不污染宿主机状态。
4. 可复现性：按文档步骤可重复得到一致结论。

## 通过判定模板

- 验证目标：
- 执行环境：
- 执行步骤：
- 实际结果：
- 预期结果：
- 判定结论（通过/不通过）：
- 备注：

## 最近一次自检

- 验证目标：cold-start helloworld（仅校验镜像可用与 auth 文件 COPY 生效）
- 执行环境：`meta/docker/Dockerfile` 构建镜像 `cowork-coldstart:hello`
- 执行步骤：`docker build -f meta/docker/Dockerfile -t cowork-coldstart:hello .`；`docker run --rm cowork-coldstart:hello sh -lc "opencode --version && opencode auth list"`
- 实际结果：`opencode --version` 返回 `1.2.16`；`auth list` 读取到 `~/.local/share/opencode/auth.json` 且显示 1 条凭据
- 预期结果：容器内通过 COPY 注入 auth 文件并被 opencode 正常识别
- 判定结论（通过/不通过）：通过
- 备注：本轮未使用 `-v` / bind mount，本地仓库未出现容器内数据文件

## 最近一次自检（复现）

- 验证目标：cold-start helloworld 复现运行（验证可重复性）
- 执行环境：`meta/docker/Dockerfile` + `meta/docker/run-coldstart-hello.sh`
- 执行步骤：连续执行两次 `./meta/docker/run-coldstart-hello.sh`
- 实际结果：两次均返回 `opencode 1.2.16` 且 `auth list` 显示 1 条凭据
- 预期结果：重复执行得到一致结论
- 判定结论（通过/不通过）：通过
- 备注：全流程未使用本地目录挂载

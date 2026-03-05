# cowork-cli

`cowork-cli` 不是业务平台，也不是通用脚手架；它只解决知识仓库协作的冷启动一致性。
我们以最小闭环（仅 `self-update` + `clone` 命令组）为先：先稳定冷启动路径（拉起、同步、预览、贡献），再讨论扩展。

## 设计哲学

- **最小闭环优先**：先把关键路径做稳，再扩展能力面。
- **边界清晰**：只定义协作接口，不介入业务 schema 细节。
- **配置显式**：通过 `COWORK_*` 环境变量保持可复现。
- **可恢复**：命令尽量可重复执行，失败时应有明确回退路径。
- **自举迭代**：先用 `cowork` 验证 `cowork` 本身，再外溢到其他仓库。

## 边界与非目标

- 不替代目标仓库的 CI、lint、测试、发布体系。
- 不在 `cowork` 内硬编码业务目录结构与字段规则。
- 不做重平台化编排，优先保持简单、可组合。

## 顶层协作契约

`cowork` 对知识仓库采用「一硬两软」模型（1 个必需接口 + 2 个推荐接口）：

- **硬约束**：`.meta.json` 必须存在，作为元信息索引与摘要入口。
- **强烈推荐**：`ci.validate` 作为仓库验证入口。
- **强烈推荐**：`resources/` 作为未结构化高价值材料归档区。

治理语义：硬约束追求跨仓库稳定接口；推荐项保留演进弹性。只有当某项在自举验证中长期稳定、失败路径可恢复、且跨仓库可迁移时，才考虑升级为硬约束。

## 演进路径与适用对象

`cowork` 面向知识仓库（knowledge repo，即承载知识资产及其生成流程的仓库）。

- 先在本仓库定义规则并执行冷启动（cold-start）验证。
- 通过验收与评估结果反向修正规则。
- 规则在本仓库稳定后，再推广到其他知识仓库。

这就是“左脚踩右脚”迭代：规则来自实践，不来自纸面假设。

## 实践入口（极简）

```bash
cowork --help
cowork clone --help

export COWORK_CLONE_REPO_URL="https://github.com/your-org/your-knowledge-repo.git"
cowork clone init
cowork clone metadata
```

命令分组可按任务理解：

- 更新：`cowork self-update`
- 同步：`cowork clone init` / `cowork clone update` / `cowork clone version`
- 观察：`cowork clone metadata` / `cowork clone preview`
- 协作：`cowork clone contribute` / `cowork clone resource`

补充：

- `cowork clone contribute` / `cowork clone resource` 是 guide-only 命令：只打印建议，不做状态变更。
- `cowork clone version` 的远端 `package.json` 地址可通过 `COWORK_CLONE_PACKAGE_URL` 覆盖；远端不可达时命令会失败退出。

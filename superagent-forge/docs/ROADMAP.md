# 实施与验收任务

当前阶段 F0：Workspace、领域模型、依赖决策与可验证边界。完成情况以 ACCEPTANCE 实测为准。以下为后续任务定义，不表示已派发给外部 Worker 或已经实现。

| 阶段 | 实施内容 | 独立验收门槛 |
| --- | --- | --- |
| F1 存储 | 评估 Pure Rust SQLite 兼容引擎，实现 Repository、事务与迁移 | 空库升级、CAS 冲突、回滚、约束、崩溃重启；审计无 C 引擎构建依赖 |
| F2 工作区和进程 | Git WorktreeManager、Runtime supervisor、Tokio、env 白名单、网络/文件隔离、资源限制 | 假 Worker 的成功/失败/超时/取消；清理后无存活孙进程；跨 Task 隔离与不能写主区 |
| F3 Worker | 实现 OMP 和 DSH AgentExecutor，验证实际 CLI 版本与协议 | 两种真实可选 CLI 各完成启动/等待/取消，stderr/坏 JSON/超量日志拒绝；核心无需 Node 仍能构建 |
| F4 编排与接入 | DAG 并发队列、持久化事件/outbox、MCP stdio、可信身份映射 | 并发上限、依赖失败、重复消息、断线恢复、未知协议版本和越权调用拒绝 |
| F5 独立 Review 和合并 | 真实 Diff/测试采集、限时一次性授权、HEAD/base CAS、授权消费 | 自审拒绝、失败测试拒绝、审后改动拒绝、重复合并拒绝、主控授权后真实合并 |

F1 和 F2 完成后再实现可执行后端；不把未实施的隔离策略当作已授权的安全保障。每项 Worker 完成后由主控独立 Review，生产路径不得复用 Worker 自报验收。

本次不继续 Codex Fork 的 P0/P1 构建，不实现外部 Runtime 对 Codex 的集成。旧实验和报告保留在父目录 audit，便于追溯，不作为 Forge 验收结果。

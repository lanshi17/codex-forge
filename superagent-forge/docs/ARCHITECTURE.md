# 架构决策：独立 Pure Rust 实现

状态：按用户最新决策替代 Codex Core Minimal Fork。旧 P0/P1 审计为历史资料，停止续建；已归档并恢复该阶段对 codex-rs/Cargo.toml 的修改。本项目没有 Codex crate 依赖。

参考来自用户给定的 DSH Superagent 领域设计，不声称已完成参考仓库源码级审计，不复制其 TypeScript 实现。

## 依赖方向

| crate | 职责与当前交付 |
| --- | --- |
| domain | Source、Trigger、TaskMandate、Task、Assignment、ExecutionReport、AgentSession，任务状态转换 |
| application | Policy、可信身份角色检查、独立 Review 与显式合并授权用例 |
| orchestrator | TaskOrchestrator 组合边界；调度循环待实现 |
| agent-runtime | 生命周期与恢复 trait；进程监督待实现 |
| agent-protocol | 版本化 ExecutionRequest |
| agent-backend | 对象安全 AgentExecutor，start/wait/cancel Future 接口 |
| agent-omp / agent-dsh | 可执行文件和 argv 配置边界；尚无进程执行实现 |
| task-scheduler | DAG 排序、环和缺失依赖拒绝；并发调度待实现 |
| worktree | 独立 Worktree 的分配/释放 port；Git 命令待实现 |
| review | 主控观察的 HEAD、Diff 检查和测试退出码门禁 |
| persistence | 带版本 CAS 的 Repository trait 和初始 SQL 草案；数据库适配器待实现 |
| mcp-server | 工具契约草案；未启动 JSON-RPC/MCP transport |
| config | 严格 TOML 配置校验，拒绝未知字段 |
| cli | forge check / protocol，无执行或合并命令 |

Domain 不依赖应用、进程、存储或 UI。Application 依赖 Domain 与 Review；Review 仅依赖 Domain。Backend 依赖 Protocol 与 Domain，OMP/DSH adapter 依赖 Backend。没有循环依赖。

## 运行部署边界

主控、协议和生命周期使用 Rust；Git、可选 OMP/DSH CLI 是外部工具，通过进程协议调用。主控的编译和基础运行不要求安装这些 Worker。

若选择的外部 DSH CLI 自身需要 Node，这是该可选 Worker 的部署依赖；Rust 主控不链接它，也不因此要求所有部署安装 Node。完全禁止 Node 的部署应禁用该 Worker 并选择可用的非 Node 后端，或将该 Worker 放在独立执行环境。

当前尚无禁网、文件隔离、进程组清理或持久化恢复实现，接口和默认配置不能等同于操作系统安全边界。完成执行里程碑前不开放运行命令。

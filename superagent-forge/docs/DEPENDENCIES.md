# 依赖选型与 Pure Rust 边界

基线使用 Rust 2024、Cargo resolver 3；声明 MSRV 1.85。当前实际测试工具链见 ACCEPTANCE，尚不声称已在 MSRV 或所有平台完成测试。Cargo.lock 固定此次解析结果。

| 能力 | 决策 | 状态 |
| --- | --- | --- |
| 模型序列化 | serde、serde_json | 已接入 |
| ID | uuid v4，新类型区分 Task/Assignment/Report/Session | 已接入 |
| 错误 | thiserror | 已接入 |
| 配置 | toml 0.9，deny_unknown_fields | 已接入 |
| Headless CLI | clap 4 | 已接入 |
| Async 执行 | 后端接口用 std::future；执行阶段优先 Tokio | Tokio 待接入；避免此阶段空载 Runtime |
| Git Worktree | std::process::Command/Tokio Command，直接 argv | port 已定义，执行待实现 |
| SQLite 持久化 | Repository port + SQL schema，评估 Turso 本地引擎 | port 已定义，未引入引擎 |
| MCP | 优先评估 Rust rmcp；JSON-RPC stdio 传输 | 契约草案，未链接 SDK |
| 日志 | tracing + 结构化脱敏层 | 执行阶段接入 |

## SQLite 与 Pure Rust 的取舍

SQLite 官方引擎以 C 实现。rusqlite/sqlx 的 SQLite 驱动最终依赖该引擎，不能把它们描述为严格的全 Rust 依赖链。本阶段不加入 libsqlite3-sys 或 bundled C SQLite，也不以文件 JSON 冒充 SQLite。

候选为 Rust 编写的 SQLite 兼容引擎 [Turso](https://github.com/tursodatabase/turso)。本次查询 crates.io 得到 `turso 0.9.0-pre.2`，属于预发布版；不直接冻结为生产依赖。该版本默认含 mimalloc，严格 Pure Rust 评估必须关闭默认特性，并审计所有传递依赖，不能仅凭顶层 crate 用 Rust 就宣布全链路通过。

采纳门槛：在无 C/C++ 编译器的构建环境验证特性组合；验证所需 SQLite 文件/SQL 兼容性、事务回滚、外键、唯一约束、CAS 并发写冲突、断电/进程崩溃恢复、迁移幂等和目标平台支持。WAL/多进程语义按该引擎真实支持情况验收，不能照搬官方 SQLite 的保证。

如果候选无法通过，不悄悄换回 C 引擎；保留 port，另行形成兼容性与依赖约束决策。当前明确的限制是：尚无可用的 SQLite 后端，尚无重启恢复功能。

## 依赖验收

执行 `cargo metadata --locked --format-version 1`，确认全部本地包只来自本目录，外部包来自 crates.io，无 Codex/DSH Git 源、Node 包或 JS 构建步骤。未来新增依赖须重跑审计；当前小依赖图的通过不能外推至未来引擎或 SDK。

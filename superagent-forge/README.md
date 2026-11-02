# Superagent Forge

独立 Rust 2024 多 Agent 编排平台的架构与领域模型基线。
只借鉴 [DeepSeek DSH Superagent](https://github.com/deepseek-ai/dsh-superagent-prototype) 的领域概念；不 Fork Codex，不链接 DSH Runtime 或 SDK。

当前交付：15 个 Cargo crate、可验证的任务状态机与权限用例、Worker trait、配置校验 CLI、协议/持久化接口及开发路线。**尚未实现 Worker 子进程、Git Worktree 创建、SQLite 后端、MCP 服务或 Git 合并。**

```sh
cd superagent-forge
cargo test --workspace --all-targets --locked
cargo run --locked -p forge-cli -- check --config config/default.toml
cargo run --locked -p forge-cli -- protocol
```

这些命令不需要 Node.js、OMP、DSH CLI 或模型凭据。`protocol` 输出预留工具列表，每项 `implemented=false`，不启动服务器。

本目录有独立 Cargo Workspace 与 Cargo.lock，可以单独复制并构建；父目录历史 Codex 源码和审计文件不参与依赖解析。

- [架构与边界](docs/ARCHITECTURE.md)
- [领域模型与状态机](docs/DOMAIN-MODEL.md)
- [协议设计](docs/TASK-PROTOCOL.md)
- [权限与隔离](docs/SECURITY.md)
- [依赖决策](docs/DEPENDENCIES.md)
- [构建和部署](docs/DEPLOYMENT.md)
- [后续实施与验收任务](docs/ROADMAP.md)
- [本阶段验收](docs/ACCEPTANCE.md)

# 构建与部署

当前仅部署架构验证 CLI。Rust 主控不要求 Node、DSH Runtime、OMP 或模型 API Key。

```sh
cd superagent-forge
cargo build --workspace --locked
cargo test --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo fmt --all -- --check
cargo run --locked -p forge-cli -- check --config config/default.toml
cargo run --locked -p forge-cli -- protocol
```

default.toml 的绝对路径是部署示例。check 不创建目录、数据库或进程；配置错误返回非零退出码。network_allowed=false 当前仅作为待执行策略校验，不声称机器已经禁网。

未来独立部署目录应包含 forge、配置、数据库和受控 Worktree 根；Worker 置于权限受限环境。OMP/DSH CLI 是可选外部后端，按其自身安装要求部署。选择 DSH 不改变 Rust 主控与 SDK 的依赖关系。

SQLite 恢复、完整 MCP stdio 服务、Worker 执行和 Review 后合并尚不可部署。不要将本阶段作为生产可用版本发布。父仓库遗留 Node 配置和 Codex 文件不属于本 Workspace 的构建输入，移出本目录进行独立构建亦应通过。

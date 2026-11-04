# CI 验收规则

只有交付候选提交对应的 GitHub Actions `blocking-ci` 运行完成且 conclusion 为 `success`，才能标记 CI 合格。必须给出运行链接和完整提交 SHA；较早提交的成功、本地测试通过、已推送、queued、skipped 或 cancelled 都不能替代当前候选的远端成功。若候选提交继续变化，需重新验收。

`CI required` 汇总作业要求每一个依赖显式成功。新增 `forge-ci` 是其必需依赖，检查 btae-eval 与 superagent-forge 两个独立 Rust Workspace：固定 Rust 1.95.0、仓库格式规则、Clippy 零警告、全部目标测试和干净工作树。BTAE-Eval 在 Ubuntu 上实际安装 Bubblewrap 并验证 user namespace，隔离测试不能静默跳过。CI 不需要模型密钥，不执行付费模型调用。

仓库 Prettier 检查覆盖 docs/active 文档。格式脚本仅将 audit/ 中的历史证据和 btae-eval/repositories 中的冻结样例排除在重写范围之外：前者按历史源码摘要留存，后者由 BTAE-Eval 测试校验 snapshot 清单。当前维护的 Rust 源码与 Python 工具仍接受格式检查，格式选择逻辑有正反例回归测试。

2026-11-04 排查发现：437dd3f98080107429537885638e2f18eea3d0c2 的 [blocking-ci](https://github.com/lanshi17/codex-forge/actions/runs/41614407840) 失败，包含真实格式问题与 GitHub 账户付款/支出限额导致的作业未启动。修复格式不代表账户问题已经解除。此修复以候选分支交付，所有必需检查通过前不合并到 main；不删除原有跨平台任务，也不将失败改成可忽略。

候选 84d48e44e71f28c5234871840060997f25d02564 的 [blocking-ci](https://github.com/lanshi17/codex-forge/actions/runs/41763111857) 已确认 repo-checks 和 superagent-forge 通过。BTAE-Eval 的畸形流测试因外层 500 毫秒耗时断言失败；后续修复让假 Worker 持续运行，改为验证协议错误、信号终止、管道关闭和任务期限内结束，避免将二进制哈希与共享 runner 调度当作性能门禁。修复候选 69bf40f7fde3b39f30c4411cd443d72eeaf83eb0 的[远端检查](https://github.com/lanshi17/codex-forge/actions/runs/41763744466)已确认两个独立 Workspace 和 repo-checks 通过，但当时仍存在 `Required runner group 'codex-forge-runners' not found` 及付款/支出限额两类启动阻塞。

运行器修复将 PR、合并后检查及发布工作流中的仓库专属私有组/标签替换为同平台、同架构的 GitHub 托管标签，见[运行器映射](../.github/workflows/README.md#hosted-runners)。无需创建 `codex-forge-runners`。Windows ARM64 的测试仍在原生 ARM64 上执行，归档仍在 x64 上构建；检查步骤、目标、分片及汇总门禁保持原样。

候选 3d8c4e63ca9b02848bfda8acddda13fb9c8d63e8 的[远端运行](https://github.com/lanshi17/codex-forge/actions/runs/41764809761)确认 Windows 和 SDK 已获得标准托管运行器，Python SDK 安装测试通过，缺失组导致的启动问题已解决。该轮最终 cancelled，不能据此宣告 CI 合格。Windows 构建另因缺少 MSVC 许可确认而失败；MODULE.bazel 要求由已接受条款的用户显式确认。后续配置仅在 Windows 上转发管理员显式设置的仓库变量 `BAZEL_MSVC_RUNTIME_VISUAL_STUDIO_EULA=1`，未设置时保持原有许可检查。

macOS 后续修复使用相同 ARM64 架构的标准 `macos-15` 替代收费 `macos-15-xlarge`，原有 Intel 选择及交叉编译目标不变。所有后续修复仍需新候选的远端检查通过；旧提交的成功、作业已启动或本地配置验证均不能替代完整 CI 验收。

用户已明确接受 Microsoft 条款并授权仅在 Windows CI 启用确认，仓库 Actions 变量已设置为 `1`。候选 2666f4578540f321e653e3df505414105f7b10b3 的[第 2 次运行](https://github.com/lanshi17/codex-forge/actions/runs/41796235406/attempts/2)中，Fence 和 Forge 检查通过，但共享 action 的许可步骤仍被跳过；后续改为工作流通过 `msvc-runtime-eula` 输入显式传值，并覆盖嵌套的 Windows lint 路径。未设置确认值时仍不导出许可变量。

同轮 BTAE 的 200 毫秒超时测试在准备阶段已耗尽总预算，因此没有启动 Worker。后续断言同时覆盖准备阶段零次执行和运行阶段一次执行，两者均不得重试；独立分类回归与后代进程测试分别验证准备超时，以及运行中的信号终止和管道关闭。准备、执行、封存继续计入同一个总预算。此处是 CI 回归修复，不构成正式 Pilot 实验结果。

工程 CI 合格与研究可以开跑是两个独立条件。即使 CI 通过，正式五组 Pilot 仍须通过方法忠实度、独立真值、隐藏评估、资源预算和环境冻结等 readiness 门禁，见[路线图](../btae-eval/protocol/ROADMAP.md)。

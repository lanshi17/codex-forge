# CI 验收规则

只有交付候选提交对应的 GitHub Actions `blocking-ci` 运行完成且 conclusion 为 `success`，才能标记 CI 合格。必须给出运行链接和完整提交 SHA；较早提交的成功、本地测试通过、已推送、queued、skipped 或 cancelled 都不能替代当前候选的远端成功。若候选提交继续变化，需重新验收。

`CI required` 汇总作业要求每一个依赖显式成功。新增 `forge-ci` 是其必需依赖，检查 btae-eval 与 superagent-forge 两个独立 Rust Workspace：固定 Rust 1.95.0、仓库格式规则、Clippy 零警告、全部目标测试和干净工作树。BTAE-Eval 在 Ubuntu 上实际安装 Bubblewrap 并验证 user namespace，隔离测试不能静默跳过。CI 不需要模型密钥，不执行付费模型调用。

仓库 Prettier 检查覆盖 docs/active 文档。格式脚本仅将 audit/ 中的历史证据和 btae-eval/repositories 中的冻结样例排除在重写范围之外：前者按历史源码摘要留存，后者由 BTAE-Eval 测试校验 snapshot 清单。当前维护的 Rust 源码与 Python 工具仍接受格式检查，格式选择逻辑有正反例回归测试。

2026-11-04 排查发现：437dd3f98080107429537885638e2f18eea3d0c2 的 [blocking-ci](https://github.com/lanshi17/codex-forge/actions/runs/41614407840) 失败，包含真实格式问题与 GitHub 账户付款/支出限额导致的作业未启动。修复格式不代表账户问题已经解除。此修复以候选分支交付，所有必需检查通过前不合并到 main；不删除原有跨平台任务，也不将失败改成可忽略。

候选 84d48e44e71f28c5234871840060997f25d02564 的 [blocking-ci](https://github.com/lanshi17/codex-forge/actions/runs/41763111857) 已确认 repo-checks 和 superagent-forge 通过。BTAE-Eval 的畸形流测试因外层 500 毫秒耗时断言失败；后续修复让假 Worker 持续运行，改为验证协议错误、信号终止、管道关闭和任务期限内结束，避免将二进制哈希与共享 runner 调度当作性能门禁。该修复仍须在新候选上通过远端 CI。此轮还确认存在 `Required runner group 'codex-forge-runners' not found` 及付款/支出限额两类基础设施阻塞；仓库管理员须恢复所需运行器和账户额度，才能完成全部必需检查。

工程 CI 合格与研究可以开跑是两个独立条件。即使 CI 通过，正式五组 Pilot 仍须通过方法忠实度、独立真值、隐藏评估、资源预算和环境冻结等 readiness 门禁，见[路线图](../btae-eval/protocol/ROADMAP.md)。

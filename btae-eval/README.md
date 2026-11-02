# BTAE-Eval：工程方法实证评估准备包

状态：**DRAFT / NOT RUN**。本次交付实验协议、五组操作流程草案、24 条候选连续变更任务、4 个可运行的小型诊断仓库、Rust 排程/指标计算器、Linux 隔离诊断运行器、DSH 离线事件适配器及反例测试。没有正式五组 Agent 比较结果，也没有“BTAE 优于 SDD”的结论。另有真实 DSH 工程派发和离线事件适配器验收，独立于研究数据。

评估工具使用 Rust 2024；Python 只用于被测代码样例。工程是独立 Cargo Workspace，不依赖 Codex、DSH SDK 或 Forge 尚未实现的 Runtime。正式运行选择 ark / deepseek-v4.1-flash、统一 DSH headless，是用户已确认的配置；尚未冻结模型不可变版本、容器和 Profile 摘要。

```sh
cd btae-eval
cargo test --locked --offline
cargo run --locked -- validate datasets/pilot.json
cargo run --locked -- plan datasets/pilot.json
cargo run --locked -- check-ready datasets/pilot.json  # 预期非零：列出未完成的开跑条件
cargo run --locked -- score examples/scorer-fixture.json
cargo run --locked -- bootstrap examples/bootstrap-fixture.json
```

score/bootstrap 示例是人工构造的评分器夹具，不能当作研究结果。plan 只输出计划，不执行任何模型、CLI 或网络请求。新增 diagnose 仅启动受隔离的假 Worker，绝不调用模型；它的退出成功只表示诊断记录已收集，须查看 outcome 字段。

| 材料 | 用途 |
| --- | --- |
| [研究协议](protocol/STUDY.md) | 比较对象、主要终点、统计单位、失败规则、成本和判定标准 |
| [机制规格](protocol/METHOD.md) | M1–M6 的输入/输出/约束与消融边界 |
| [基线操作流程](baselines/README.md) | 五组差异、共同信息和实施忠实度 |
| [执行与隔离](protocol/EXECUTION.md) | DSH 配置冻结、运行记录、隐藏评估与禁止泄漏 |
| [标注手册](protocol/ANNOTATION.md) | 独立双人 Ground Truth、盲审和实体匹配 |
| [依次实施任务](protocol/ROADMAP.md) | 运行真实 Pilot 前的剩余交付 |
| [候选数据](datasets/pilot.json) | 4 仓库 × 6 轮 × 5 组 × 3 重复 = 360 次计划执行 |
| [运行器验证](reports/RUNNER-VALIDATION.md) | Linux 进程、预算、取消、隔离、快照与完整性验证；真实实验仍未运行 |
| [DSH 适配器验收](reports/DSH-ADAPTER-ACCEPTANCE.md) | 真实 headless 调用、独立复验、缓存统计更正与超时记录；不是五组效果实验 |
| [准备阶段报告](reports/PREPARATION.md) | 本次验证和未解决事项；没有效果数据 |

当前任务只有 small 合成仓库、6 轮、单 Agent；不等于用户设想的中型项目、20–30 轮、多 Agent 或外部真实仓库验证。24 个 task 不是 24 个独立仓库；360 次执行形成 60 条分组轨迹，推断时仍要尊重 4 个仓库簇。

evaluator-private 在开发 checkout 中可被读取；目录名不构成隔离。真实 Worker 只允许获得导出的公开输入，不能挂载本目录。所有候选真值仍为 agent-authored 草稿，必须独立复核后冻结。

## Linux 运行器诊断

需要 `/usr/bin/bwrap`、可用的 user/pid/network/mount namespaces；不允许失败后回退成宿主机无隔离执行。

```sh
cargo build --bins --locked --offline
mkdir -p runs
cargo run --locked -- diagnose --fixture-worker target/debug/btae-fixture-worker --output runs/smoke-01 --scenario retry
cargo run --locked -- verify-diagnostic runs/smoke-01 datasets/pilot.json
```

输出目录必须不存在，已有结果不可覆盖。run 的 data_kind 为 runner_diagnostic_only，hidden_test_result 与 empirical_task_success 恒为 null；诊断不能反序列化成正式 Observation。默认允许诊断第1轮公开种子，不将后续轮次偷偷重置到初始代码。

诊断的 token 是假 Worker 自报的计数，验证器检测累计超额后终止进程；这不是服务商硬配额或可信账单。已实现 [DSH 离线 JSONL 适配器](protocol/DSH-ADAPTER.md)，支持流式解析、缓存分项、字段覆盖和失败终态；没有接入真实 DSH 进程收集、可信账单或六机制干预。

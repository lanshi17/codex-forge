# 任务协议 v1（设计基线）

协议分为主控调用协议、后端执行协议。MCP/JSON-RPC 供主控接入；OMP/DSH 的子进程协议由各 Adapter 翻译，不假定两者有相同的 JSONL 格式。

ExecutionRequest 包含版本、主控生成的 Assignment、TaskMandate 和 prompt。后端 trait 使用 start(request)、wait(session)、cancel(session)，返回 AgentSession 和 ExecutionReport；Worker 不接收 Task 聚合的写权限，也不能提交 Approved 状态。

ExecutionReport 的字段是 id、task_id、assignment_id、exit_code、summary、head_commit、changed_files。主控必须从受控 Session 核对身份、版本、Task/Assignment 关联，限制输入字节数与路径范围，再入库。Worker 退出码 0 不等于 Review 通过。

后续线路协议要求：JSON-RPC 2.0 request id、显式协议版本、不支持版本时拒绝、消息大小限制、错误码、取消与断线语义、重复消息幂等。当前只实现 Rust ExecutionRequest/Report 数据类型，尚未实现线路解析、握手或幂等存储。

预留 MCP 工具为 forge_task_create、forge_task_status、forge_assignment_cancel；CLI protocol 将它们标记为 implemented=false。不向 Worker 暴露 Review 或 Merge 工具。MCP 的 initialize、tools/list、tools/call 和 stdio framing 将随服务器实现一起验收，不能把普通 JSON 输出称作完整 MCP 服务。

任务接受、授权、执行报告、Review、合并授权均应生成持久化审计事件；最终与 Task snapshot 在同一数据库事务中写入 outbox。现有 SQL 仅为表结构草案。

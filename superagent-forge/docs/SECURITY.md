# 权限和执行安全

三个独立门禁：任务接受与执行授权分离；执行结果与验收分离；验收与合并授权分离。Worker 不可修改 Policy、数据库或任务状态快照。

当前已实现：授权角色检查，Mandate 的后端与过期限制，Worker 提交身份检查，Reviewer 独立身份检查，Report/Assignment/HEAD 绑定，host evidence 门禁，未通过 Review 拒绝合并授权，取消后拒绝迟到结果。

当前尚未实现身份认证、进程执行或 OS sandbox。角色检查依赖可信调用方传入的 PrincipalId；这不是完整网络服务的鉴权实现。不得将库 API 或反序列化快照直接暴露给 Worker。

## 执行阶段必须落实

- 每个 Assignment 一个真实 Git Worktree，命令 cwd 从 Lease 获取；不同 Task 的目录/分支在持久化层也必须唯一。
- 环境 env_clear 后仅按全局 Policy 与 Mandate 交集传递，凭据按指定后端注入，日志中不记录值。
- 网络默认禁止。配置值只是策略声明，执行必须有 Linux namespace/适用平台 sandbox 等实际强制边界，防止 Worktree 内进程访问主工作区、数据库和共享 Git 管理目录。
- 可执行文件来自可信配置，参数作为数组，prompt 经 stdin；不经过 sh -c，不在 argv 放密钥。
- 限制进程时间、并发、日志和协议消息体大小；取消与超时终止整个进程组并回收子进程；恢复时不凭 PID 单独认领进程。
- Review 在主控收集真实 Diff、HEAD 和测试结果。合并前重新校验被审查 HEAD、目标分支基线、Policy、一次性授权及 CAS 状态；Worker 不能调用合并入口。

当前 MergeAuthorization 是应用结果对象，尚无签名、到期、一次性消费、Git HEAD CAS 或真实合并动作。后续实现这些条件之前不将它视为可在不可信边界使用的凭据。

主控代码禁止 unsafe，但传递依赖的 unsafe、OS 系统库接口和外部可执行文件属于单独审计范围。Pure Rust 不代表无需操作系统隔离。

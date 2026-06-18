# Security Policy

`winux` 会处理命令执行、PATH 扫描、shell 集成和路径转换，因此安全问题需要谨慎处理。

## 支持版本

当前项目仍处于 `0.x` 初始化阶段。默认只维护最新的 `main` 分支。

## 报告安全问题

请不要在公开 issue 中披露可利用的安全漏洞。

推荐通过 GitHub 的私有安全通告功能报告。如果该功能尚未开启，请先创建一个不包含利用细节的 issue，说明你希望私下报告安全问题。

## 高风险区域

请特别关注：

- shell escaping / quoting。
- `cmd.exe`、PowerShell、`.bat`、`.cmd` 调用。
- 不可信输入进入 `winux run` 或未来的 shell mode。
- PATH 劫持和命令名冲突。
- 自动写入用户 shell profile。

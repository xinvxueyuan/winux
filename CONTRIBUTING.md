# Contributing

感谢你关注 `winux`。

## 开发环境

需要安装 Rust stable。推荐使用 `rustup` 管理工具链。

```bash
rustup toolchain install stable
rustup component add rustfmt clippy
```

## 本地检查

提交前请运行：

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

## 提交建议

推荐使用简洁的 Conventional Commits 风格：

- `feat:` 新功能
- `fix:` 修复问题
- `docs:` 文档更新
- `chore:` 工程配置或维护
- `ci:` CI/CD 相关修改
- `refactor:` 重构

## 当前优先级

当前优先补齐 Windows CLI 工作流能力：

1. `doctor` 环境诊断。
2. `which` 命令来源解释。
3. `run` 命令代理。
4. Windows / Git Bash / MSYS2 / WSL 路径转换。
5. PowerShell 集成。

## Pull Request

请在 PR 中说明：

- 变更内容。
- 本地验证命令。
- 是否影响 Windows 行为。
- 是否涉及命令执行、PATH、shell escaping 或安全风险。

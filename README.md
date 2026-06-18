# winux

`winux` 是一个用 Rust 开发的 Windows 类 Linux CLI 工作流管理器。

项目目标不是重新实现所有 Unix 命令，而是帮助 Windows 开发者发现、解释并稳定运行 Linux 风格命令行工具。后续可集成 Windows 原生命令、Microsoft Coreutils for Windows、uutils、Git Bash、MSYS2、BusyBox 和 WSL。

## 当前状态

仓库处于基础设施初始化阶段。第一个里程碑聚焦三个小而有用的命令：

- `winux doctor`：检查当前命令行环境。
- `winux which <cmd>`：列出 `PATH` 中匹配到的可执行候选项。
- `winux run <cmd> [args...]`：执行命令并透传退出码。

## 使用示例

```bash
cargo run -- doctor
cargo run -- which grep
cargo run -- run echo hello
```

## 本地开发

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

## 建议路线图

1. 完善 `doctor`：识别 PowerShell、CMD、Windows Terminal、Coreutils、Git Bash、MSYS2、WSL。
2. 完善 `which`：解释 PowerShell alias / function / cmdlet / external executable 的优先级。
3. 完善 `run`：增加后端选择，例如 `--backend coreutils|gitbash|msys2|wsl`。
4. 增加路径转换：`winux path to-win|to-unix|to-wsl`。
5. 增加 shell 集成：`winux init powershell|cmd|gitbash`。

## 许可证

本项目使用 Apache License 2.0。详见 [LICENSE](LICENSE)。

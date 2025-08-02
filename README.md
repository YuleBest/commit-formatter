# Commit Formatter

一个用 Rust 编写的命令行工具，帮助用户生成符合 Angular 规范的 Git commit 消息。 

## 功能特性

- 交互式界面，引导用户逐步创建 commit 消息
- 支持完整的 Angular commit 规范
- 彩色输出，提升用户体验
- 输入验证，确保 commit 消息质量
- 可选择直接执行 git commit 命令

## Angular Commit 规范

本工具支持以下 commit 类型：

- **feat**: 新功能 (A new feature)
- **fix**: 修复 bug (A bug fix) 
- **docs**: 文档更新 (Documentation only changes)
- **style**: 代码格式 (Changes that do not affect the meaning of the code)
- **refactor**: 重构 (A code change that neither fixes a bug nor adds a feature)
- **perf**: 性能优化 (A code change that improves performance)
- **test**: 测试 (Adding missing tests or correcting existing tests)
- **build**: 构建系统 (Changes that affect the build system or external dependencies)
- **ci**: CI 配置 (Changes to our CI configuration files and scripts)
- **chore**: 其他杂务 (Other changes that don't modify src or test files)
- **revert**: 回滚 (Reverts a previous commit)

## 安装

### 方式一：下载预编译的二进制文件

从 [Releases](https://github.com/YuleBest/commit-formatter/releases) 页面下载适合你操作系统的预编译二进制文件：

- **Windows**: `commit-formatter-windows-x64.exe` 或 `commit-formatter-windows-x86.exe`
- **Linux**: `commit-formatter-linux-x64` 或 `commit-formatter-linux-arm64`
- **macOS**: `commit-formatter-macos-x64` 或 `commit-formatter-macos-arm64`

### 方式二：从源码编译

确保你已经安装了 Rust 和 Cargo，然后克隆此仓库：

```bash
git clone https://github.com/YuleBest/commit-formatter.git
cd commit-formatter
cargo build --release
```

编译完成后，可执行文件位于 `target/release/` 目录下。

### 加入 PATH（可选）

将 `target/release/` 目录添加到系统的 PATH 环境变量中，即可在任意位置直接调用 `commit-formatter` 命令，你可以将可执行文件改名为 `cf.exe` 或其他你喜欢的简称，方便你的使用。

## 使用方法

### 交互式模式

直接运行程序进入交互式模式：

```bash
commit-formatter
```

或者显式指定交互式模式：

```bash
commit-formatter --interactive
commit-formatter -i
```

### 使用流程

1. **选择 commit 类型**: 从预定义的 Angular 规范类型中选择
2. **输入作用域** (可选): 指定此次更改影响的模块或组件
3. **输入简短描述** (必填): 用现在时态描述此次更改，不超过 50 个字符
4. **输入详细描述** (可选): 支持多行输入，详细解释更改的动机和实现方式
5. **输入破坏性变更** (可选): 描述不兼容的 API 变更
6. **输入关联 issue** (可选): 关联相关的 issue 编号

### 多行详细描述

工具支持多行详细描述输入：
- 逐行输入详细描述内容
- 输入空行结束多行输入
- 生成的 git 命令会为每行使用单独的 `-m` 参数
- 确保 commit 消息格式正确且易于阅读

### 示例输出

```
feat(auth): add user authentication system

Implement JWT-based authentication with login and logout functionality.
Add middleware for protecting routes and user session management.

BREAKING CHANGE: Authentication is now required for all API endpoints

Closes #123
Closes #456
```

对应的git命令：
```bash
git commit -m "feat(auth): add user authentication system" -m "" -m "Implement JWT-based authentication with login and logout functionality." -m "Add middleware for protecting routes and user session management." -m "" -m "BREAKING CHANGE: Authentication is now required for all API endpoints" -m "" -m "Closes #123" -m "Closes #456"
```

## 依赖项

- `clap`: 命令行参数解析
- `inquire`: 交互式命令行界面
- `colored`: 彩色终端输出

## CI/CD 和自动构建

本项目使用 GitHub Actions 进行持续集成和自动构建：

### 工作流程

1. **CI 工作流** (`.github/workflows/ci.yml`):
   - 在每次推送和 Pull Request 时触发
   - 运行代码格式检查、Clippy 静态分析和测试
   - 在 Ubuntu、Windows、macOS 三个平台上进行构建测试

2. **交叉平台构建工作流** (`.github/workflows/build.yml`):
   - 支持多个目标平台的交叉编译
   - 自动生成各平台的可执行文件
   - 在创建 tag 时自动创建 GitHub Release

### 支持的平台

- **Windows**: x86_64 和 i686 架构
- **Linux**: x86_64 (glibc/musl) 和 ARM64 架构
- **macOS**: x86_64 和 ARM64 (Apple Silicon) 架构

## 贡献

欢迎提交 Issue 和 Pull Request 来改进这个工具！

在提交代码前，请确保：
- 运行 `cargo fmt` 格式化代码
- 运行 `cargo clippy` 检查代码质量
- 运行 `cargo test` 确保测试通过

## 许可证

[PolyForm Noncommercial License 1.0.0](./LICENSE.md)（[中文简体版本](./LICENSE-zh-Hans.md)）
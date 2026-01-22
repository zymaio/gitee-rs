# gitee-rs

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust: 1.75+](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

> **⚖️ 免责声明 (Legal Disclaimer)**
>
> 本项目（"gitee-rs"）是由社区开发者维护的**第三方开源工具**，并非 Gitee (OSChina/开源中国) 的官方产品。
> 1. 本项目与 Gitee 官方无任何隶属、关联、授权、背书关系。
> 2. "Gitee" 商标所有权归其各自持有者所有。本项目仅用于描述与该平台 API 的集成。
> 3. 用户使用本项目产生的任何后果（包括但不限于数据丢失、API 频率超限等）由用户自行承担，作者不承担任何法律责任。
> 4. 本项目仅作为工具使用，不收集、不存储、不上传任何用户的个人数据或访问令牌 (Token) 到第三方服务器。

---

## 🚀 项目简介



Gitee Tools 是一个高性能、类型安全的 Rust 工具集，旨在通过 Gitee API 简化自动化开发流程。



- **GitHub**: [github.com/zymaio/gitee-rs](https://github.com/zymaio/gitee-rs)

- **Gitee**: [gitee.com/fourthz/gitee-rs](https://gitee.com/fourthz/gitee-rs)



本项目深度参考并实现了官方

 [mcp-gitee (Go)](https://gitee.com/oschina/mcp-gitee) 的所有功能，并利用 Rust 的异步特性提供了更强的健壮性。

### 核心优势 (Key Advantages)
- **🚀 零依赖单文件**: 编译后生成独立二进制文件，无需安装运行时环境，分发极简。
- **🛡️ 内存安全与极致性能**: 基于 Rust 编写，资源占用极低，适合作为 Sidecar 长时间运行。
- **🌍 国际化支持**: 命令行帮助与运行日志支持中英文自动切换（识别 `LANG` 环境变量）。
- **🔌 双协议支持**: 同时支持 `stdio` (本地本地 IDE) 和 `SSE` (网络/远程集成)。
- **📂 自动化配置**: 内置 `install` 指令，一键生成 AI 辅助工具所需的 JSON 配置文件。

### 核心组件
- **gitee-rs**: 核心 API 客户端库，支持异步请求、强类型反序列化及自定义 API 基地址。
- **gitee-mcp**: 兼容 Model Context Protocol (MCP) 的服务器。
- **gitee-cli**: 命令行工具，提供直观的仓库、问题、拉取请求管理接口。

## ✨ 主要功能

- **Issues & PRs**: 完整的生命周期管理（创建、列表、详情、更新、评论、关闭/合并）。
- **Repository**: 信息查询、个人/组织仓库创建、Fork、全站搜索。
- **Files & Contents**: 文件内容读取、目录树列出、基于内容的全局代码搜索。
- **Notifications**: 实时获取用户通知。
- **Labels**: 标签的增删改查。
- **Releases**: 版本发布管理。

## 🛠️ 安装与分发

### 1. 直接下载 (推荐)
本项目为 **零依赖单二进制文件**。你可以直接从 GitHub/Gitee 的 **Releases** 页面下载对应操作系统的压缩包，解压后即可运行：
- **Linux/macOS**: 下载后建议移动到 `/usr/local/bin` 并授予执行权限 `chmod +x`。
- **Windows**: 下载后解压到任意文件夹，并将其路径添加到系统环境变量 `Path` 中。

### 从源码构建
如果你本地有 Rust 环境：
```bash
# GitHub
git clone https://github.com/zymaio/gitee-rs.git
# Gitee
git clone https://gitee.com/fourthz/gitee-rs.git

cd gitee-rs
cargo build --release
```
编译产物位于 `target/release/gitee` (CLI) 和 `target/release/gitee-mcp` (MCP)。

### 3. 开发集成 (Crates.io)
如果你是 Rust 开发者，想在项目中使用 SDK：
```toml
[dependencies]
gitee-rs = "0.1.0"
```


## ⚙️ 配置说明

### 环境变量 (推荐)
- `GITEE_ACCESS_TOKEN`: Gitee API 访问令牌（必须）。
- `GITEE_API_BASE`: Gitee API 基地址（默认: `https://gitee.com/api/v5`）。
- `LANG`: 语言设置，支持 `zh_CN.UTF-8` 自动切换中文界面。

---

## 🤖 MCP Server 使用 (gitee-mcp)

### 1. 快速配置 (推荐)
使用内置的 `install` 命令为你的 AI 工具生成配置：
```bash
./gitee-mcp install --name my-gitee --token "你的Token"
```
这将在当前目录生成 `_cursor_config.json` 等文件，你只需将其内容复制到相应 IDE 的 MCP 设置中即可。

### 2. 传输模式
- **Stdio 模式**: 默认模式，直接与 IDE 进程通信。
- **SSE 模式**: `./gitee-mcp --transport sse --port 8000`，通过 HTTP 协议提供服务。

### 3. 高级配置
- **工具白名单**: `--enabled-toolsets "list_issues,get_repo"`
- **工具黑名单**: `--disabled-toolsets "delete_label"`

---

## 💻 CLI 使用 (gitee-cli)

`gitee-cli` 提供了丰富的子命令，你可以通过 `gitee --help` 查看完整列表。

```bash
# 查看仓库信息
gitee repo info fourthz gitee-rs-test

# 列出问题
gitee issues list

# 创建标签
gitee labels create owner repo "bug" --color "ff0000"
```

## 📜 开源协议

本项目采用 [MIT License](LICENSE) 开源。

## 👤 作者 (Author)

- **fourthz**
  - GitHub: [@zymaio](https://github.com/zymaio)
  - Gitee: [@fourthz](https://gitee.com/fourthz)

## 🤝 致谢


- 感谢 [mcp-gitee](https://gitee.com/oschina/mcp-gitee) 提供的工具定义参考。
- 感谢 Gitee 提供的开放 API。
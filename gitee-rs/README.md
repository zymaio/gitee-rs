# gitee-rs

A high-performance, type-safe Gitee API client library for Rust.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust: 1.75+](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

> **⚖️ 免责声明 (Disclaimer)**: 本项目为第三方开发，并非 Gitee (OSChina) 官方产品。

## ✨ 功能特性

- **全面覆盖**: 支持 Issues, Pull Requests, Repositories, Users, Notifications, Files, Labels, Releases 等。
- **异步支持**: 基于 `reqwest` 和 `tokio` 实现全异步调用。
- **健壮的数据模型**: 针对 Gitee API 的复杂标识符格式做了专门处理，减少反序列化崩溃。
- **可定制性**: 支持自定义 API 基地址，适配 Gitee 专有云场景。

## 🚀 快速开始

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
gitee-rs = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### 基础用法

```rust
use gitee_rs::GiteeClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化客户端 (从环境变量 GITEE_TOKEN 读取)
    let client = GiteeClient::new(None, None)?;

    // 获取仓库信息
    let repo = client.get_repo("owner", "repo").await?;
    println!("Repo name: {}", repo.full_name);

    // 列出 Issue
    let issues = client.list_issues().await?;
    for issue in issues {
        println!("#{} - {}", issue.number, issue.title);
    }

    Ok(())
}
```

## 🛠️ 模块概览

| 模块 | 功能说明 |
| --- | --- |
| `issues` | 问题的创建、详情、列表、更新、评论等 |
| `pulls` | 拉取请求的完整生命周期管理及差异文件查询 |
| `repos` | 仓库查询、创建（个人/组织）、Fork、搜索 |
| `files` | 读取文件内容、列出目录树、全局代码搜索 |
| `users` | 获取用户信息及用户搜索 |
| `labels` | 标签的自动化管理 |
| `releases` | 版本发布管理 |
| `notifications` | 用户通知实时拉取 |

## 📜 开源协议

本项目采用 [MIT License](../LICENSE) 开源。

# Gitee Tools 开发文档

## 项目概述

Gitee Tools 是一个综合性的 Gitee 自动化工具集，包含三个主要组件：
- **gitee-rs**：核心 API 客户端库，负责所有与 Gitee API 的交互
- **gitee-cli**：基于命令行的交互工具
- **gitee-mcp**：支持 Model Context Protocol (MCP) 的 Provider，支持 stdio 和 SSE 传输

## 架构设计

项目采用分层架构：
1. **gitee-rs**: 所有的业务逻辑、API 调用和数据模型定义。
2. **展现层**:
   - **gitee-cli**: 负责解析命令行参数并调用 core。
   - **gitee-mcp**: 负责解析 MCP 协议（JSON-RPC），通过不同的 Server 实现（stdio/SSE）与 AI Agent 通信。

## gitee-mcp 配置与使用

### 传输方式 (Transport)
支持两种主要的传输方式：
- **stdio**: (默认) 用于本地 AI 工具集成（如 Claude Desktop, Cursor, Cline）。
- **sse**: 基于 HTTP 的 Server-Sent Events，用于远程连接。

### 命令行参数
- `--token`: Gitee 访问令牌 (也可通过环境变量 `GITEE_ACCESS_TOKEN` 设置)。
- `--api-base`: Gitee API 基地址，默认 `https://gitee.com/api/v5` (也可通过环境变量 `GITEE_API_BASE` 设置)。
- `--transport`: 传输协议，`stdio` 或 `sse` (默认 `stdio`)。
- `--host`: SSE 服务监听地址 (默认 `127.0.0.1`)。
- `--port`: SSE 服务监听端口 (默认 `8000`)。
- `--enabled-toolsets`: 启用的工具列表（逗号分隔），例如 `list_issues,get_repo`。
- `--disabled-toolsets`: 禁用的工具列表（逗号分隔）。

### 示例启动命令
```bash
# 标准输入输出模式
gitee-mcp --token YOUR_TOKEN

# SSE 模式
gitee-mcp --transport sse --port 8080
```

## 测试状态

有关详细的测试状态，请参阅 [TEST_STATUS.md](TEST_STATUS.md)。

### 已实现的功能 (gitee-rs)

| 模块 | 功能 |
| --- | --- |
| Issues | 列出、详情、创建、更新、关闭、评论、列出评论 |
| Pull Requests | 列出、详情、创建、更新、关闭、合并、评论、列出评论、差异文件 |
| Repositories | 获取信息、创建个人/组织仓库、列出仓库、Fork、搜索 |
| Releases | 列出、创建 |
| Users | 获取用户信息、搜索用户 |
| Notifications | 列出通知 |
| Files | 获取文件内容、列出文件、按内容搜索 |
| Labels | 列出、创建、更新、删除 |

## API 文档

### gitee-rs 核心接口

#### GiteeClient 结构体
```rust
pub fn new(token: Option<String>, base_url: Option<String>) -> Result<GiteeClient, GiteeError>
```

#### 各模块 API
所有 API 均定义在 `gitee-rs/src/` 下的对应目录中，遵循 `pub async fn` 异步风格。

## 配置说明

### 环境变量
- `GITEE_ACCESS_TOKEN`: Gitee API 访问令牌
- `GITEE_API_BASE`: Gitee API 基地址
- `ENABLED_TOOLSETS`: 启用的工具集
- `DISABLED_TOOLSETS`: 禁用的工具集

## 开发指南

### 添加新功能
1. 在 `gitee-rs` 中实现核心 API 逻辑。
2. 在 `gitee-mcp/src/tools/definitions.rs` 中添加工具描述。
3. 在 `gitee-mcp/src/tools/dispatcher.rs` 中添加分发映射。
4. 在 `gitee-cli/src/commands/` 下添加对应的 CLI 子命令。

### 鲁棒性建议
- **数据反序列化**: 始终使用 `crate::utils::deserialize_string_or_int` 处理 ID 和 Number 字段，因为 Gitee 可能返回数字或字符串。
- **可选字段**: API 响应中的大部分字段可能为 `null`，在模型定义中应优先使用 `Option<T>`。

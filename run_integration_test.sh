#!/bin/bash

# Gitee Tools - 集成测试脚本

set -e  # Exit on any error

echo "==================================="
echo "Gitee Tools - 集成测试"
echo "==================================="

# 检查是否设置了必需的环境变量
if [ -z "$GITEE_TOKEN" ]; then
    echo "错误: 未设置 GITEE_TOKEN 环境变量"
    echo "请设置你的 Gitee 访问令牌: export GITEE_TOKEN='your_token_here'"
    exit 1
fi

if [ -z "$GITEE_DEFAULT_OWNER" ]; then
    echo "错误: 未设置 GITEE_DEFAULT_OWNER 环境变量"
    echo "请设置你的 Gitee 用户名: export GITEE_DEFAULT_OWNER='your_username'"
    exit 1
fi

# 创建测试仓库
echo "正在创建测试仓库..."
REPO_NAME="gitee-tools-test-$(date +%s)"
echo "测试仓库名称: $REPO_NAME"

# 使用 curl 创建仓库
response=$(curl -s -X POST \
    "https://gitee.com/api/v5/user/repos" \
    -H "Content-Type: application/json" \
    -H "Authorization: token $GITEE_TOKEN" \
    -d '{
        "name": "'"$REPO_NAME"'",
        "description": "Test repository for gitee-tools integration tests",
        "private": false,
        "auto_init": true
    }')

if [[ $response == *"id"* ]]; then
    echo "✓ 成功创建测试仓库: $REPO_NAME"
else
    echo "✗ 创建仓库失败: $response"
    exit 1
fi

# 设置测试仓库名称
export TEST_REPO_NAME="$REPO_NAME"

echo "开始运行集成测试..."
echo ""

# 构建项目
echo "构建项目..."
cargo build --bins

echo ""
echo "测试 CLI 工具..."

# 测试仓库信息
echo "1. 测试仓库信息..."
gitee_bin="./target/debug/gitee"
if [ ! -f "$gitee_bin" ]; then
    gitee_bin=$(find ./target/debug -name "gitee" -type f | head -n 1)
fi

if [ -f "$gitee_bin" ]; then
    echo "   使用二进制文件: $gitee_bin"
    "$gitee_bin" repo info "$GITEE_DEFAULT_OWNER" "$REPO_NAME"
    echo "   ✓ 仓库信息测试完成"
else
    echo "   构建 CLI 工具..."
    cargo build -p gitee-cli
    "$gitee_bin" repo info "$GITEE_DEFAULT_OWNER" "$REPO_NAME"
    echo "   ✓ 仓库信息测试完成"
fi

# 测试标签功能
echo ""
echo "2. 测试标签功能..."
"$gitee_bin" labels list "$GITEE_DEFAULT_OWNER" "$REPO_NAME"
echo "   ✓ 标签列表测试完成"

# 创建标签
LABEL_NAME="test-label-$(date +%s)"
"$gitee_bin" labels create "$GITEE_DEFAULT_OWNER" "$REPO_NAME" "$LABEL_NAME" --color "ff0000" --description "Test label for integration test"
echo "   ✓ 标签创建测试完成"

# 测试问题功能
echo ""
echo "3. 测试问题功能..."
"$gitee_bin" issues list
echo "   ✓ 问题列表测试完成"

# 创建问题
ISSUE_TITLE="Integration Test Issue $(date +%s)"
"$gitee_bin" issues create "$GITEE_DEFAULT_OWNER" "$REPO_NAME" "$ISSUE_TITLE" --body "This is an integration test issue."
echo "   ✓ 问题创建测试完成"

# 测试拉取请求功能
echo ""
echo "4. 测试拉取请求功能..."
"$gitee_bin" pr list "$GITEE_DEFAULT_OWNER" "$REPO_NAME"
echo "   ✓ 拉取请求列表测试完成"

echo ""
echo "==================================="
echo "集成测试完成!"
echo "测试仓库: $REPO_NAME"
echo "==================================="

# 询问是否清理测试仓库
echo ""
echo "是否删除测试仓库? (y/N)"
read -r response
if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
    echo "正在删除测试仓库: $REPO_NAME..."
    curl -s -X DELETE \
        "https://gitee.com/api/v5/repos/$GITEE_DEFAULT_OWNER/$REPO_NAME" \
        -H "Authorization: token $GITEE_TOKEN"
    echo "✓ 测试仓库已删除"
else
    echo "测试仓库 $REPO_NAME 已保留"
fi

echo ""
echo "==================================="
echo "使用说明:"
echo "==================================="
echo "要运行单元测试，请使用："
echo "  ./run_tests.sh"
echo ""
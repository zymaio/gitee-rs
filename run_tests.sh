#!/bin/bash

echo "==================================="
echo "Gitee-Tools 単元测试脚本"
echo "==================================="

# 检查是否设置了 GITEE_TOKEN
if [ -z "$GITEE_TOKEN" ]; then
    echo "警告: 未设置 GITEE_TOKEN 环境变量"
    echo "某些测试将被跳过，因为它们需要有效的 API 令牌"
    echo ""

    # 只运行不需要网络连接的单元测试
    echo "运行单元测试 (测试数据结构反序列化)..."
    cargo test -p gitee-rs --lib

else
    echo "GITEE_TOKEN 已设置，运行完整测试套件..."
    echo ""

    # 运行所有测试
    echo "运行所有测试..."
    cargo test
fi

echo ""
echo "==================================="
echo "単元测试完成!"
echo "==================================="
echo ""
echo "要运行特定测试，请使用以下命令之一："
echo "  cargo test -p gitee-rs --lib tests::test_data_structures_deserialization          # 测试数据结构反序列化"
echo "  cargo test -p gitee-rs --lib tests::test_label_structure_deserialization        # 测试标签结构反序列化"
echo "  cargo test -p gitee-rs --lib tests::test_pull_request_structure_deserialization # 测试 PR 结构反序列化"
echo "  cargo test -p gitee-rs --lib tests::test_client_creation                        # 测试客户端创建 (需要 GITEE_TOKEN)"
echo "  cargo test -- --nocapture                                                         # 运行所有测试并显示输出"
echo ""
echo "要运行集成测试，请使用："
echo "  ./run_integration_test.sh"
echo "  ./test_cli_functionality.sh"
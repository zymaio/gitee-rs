#!/bin/bash

# Gitee Tools - 统一测试管理脚本

echo "==================================="
echo "Gitee Tools - 统一测试管理"
echo "==================================="
echo ""
echo "请选择要运行的测试类型："
echo ""
echo "1) 单元测试 (Unit Tests) - 测试核心功能和数据结构"
echo "2) 集成测试 (Integration Tests) - 端到端功能测试"
echo "3) 完整测试套件 (Complete Test Suite) - 运行所有测试"
echo "4) 退出"
echo ""

read -p "请输入选项 (1-4): " choice

case $choice in
    1)
        echo ""
        echo "运行单元测试..."
        echo "=================="
        ./run_tests.sh
        ;;
    2)
        echo ""
        echo "运行集成测试..."
        echo "=================="
        ./run_integration_test.sh
        ;;
    3)
        echo ""
        echo "运行完整测试套件..."
        echo "=================="
        echo "首先运行单元测试..."
        ./run_tests.sh
        echo ""
        echo "然后运行集成测试..."
        ./run_integration_test.sh
        ;;
    4)
        echo "退出测试管理。"
        exit 0
        ;;
    *)
        echo "无效选项: $choice"
        echo "退出。"
        exit 1
        ;;
esac

echo ""
echo "==================================="
echo "所有选定的测试已完成！"
echo "==================================="
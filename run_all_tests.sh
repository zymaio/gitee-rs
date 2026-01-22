#!/bin/bash
export GITEE_TOKEN=b67fb5f1bd50b9d16abcd417bd5dfbf4
export OWNER=fourthz
export REPO=gitee-tools-test
export CLI="cargo run -q -p gitee-cli --"

report="TEST_REPORT.md"
echo "# Gitee CLI All-In-One Test Report" > $report
echo "Date: $(date)" >> $report
echo "| Category | Command | Status | Details |" >> $report
echo "|----------|---------|--------|---------|" >> $report

run_test() {
    category=$1
    cmd_name=$2
    full_cmd=$3
    
    echo "Testing $category: $cmd_name..."
    # 使用 eval 确保引号被正确解析
    output=$(eval "$full_cmd" 2>&1)
    if [ $? -eq 0 ]; then
        status="✅ PASS"
        detail=$(echo "$output" | head -n 1 | tr -d '|')
    else
        status="❌ FAIL"
        detail=$(echo "$output" | head -n 1 | tr -d '|')
    fi
    echo "| $category | $cmd_name | $status | $detail |" >> $report
}

# --- Repos ---
run_test "Repos" "list" "$CLI repo list"
run_test "Repos" "info" "$CLI repo info $OWNER $REPO"
run_test "Repos" "fork" "$CLI repo-ext fork $OWNER $REPO"

# --- Issues ---
run_test "Issues" "list" "$CLI issues list"
run_test "Issues" "create" "$CLI issues create $OWNER $REPO 'Report-Test' --body 'Generated'"
run_test "Issues" "detail" "$CLI issues-ext detail $OWNER $REPO IDL5EU"
run_test "Issues" "comment" "$CLI issues-ext comment $OWNER $REPO IDL5EU 'Test-Comment-$(date +%s)'"

# --- Pull Requests ---
run_test "PRs" "list" "$CLI pr list $OWNER $REPO"

# --- Labels ---
run_test "Labels" "list" "$CLI labels list $OWNER $REPO"
run_test "Labels" "create" "$CLI labels create $OWNER $REPO 'tmp-tag' 'ff0000'"
run_test "Labels" "update" "$CLI labels update $OWNER $REPO 'tmp-tag' --new-name 'tmp-tag-fixed'"
run_test "Labels" "delete" "$CLI labels delete $OWNER $REPO 'tmp-tag-fixed'"

# --- Users ---
run_test "Users" "info (self)" "$CLI user info"
run_test "Users" "info (other)" "$CLI user info fourthz"
run_test "Users" "search" "$CLI user search fourthz"

# --- Files ---
run_test "Files" "list" "$CLI files list $OWNER $REPO"
run_test "Files" "get" "$CLI files get $OWNER $REPO README.md"
run_test "Files" "search" "$CLI files search 'Gitee' --owner $OWNER"

# --- Releases & Notifications ---
run_test "Releases" "list" "$CLI releases list $OWNER $REPO"
run_test "Notifications" "list" "$CLI notifications list"

echo "Testing complete. Report generated at $report"
$ErrorActionPreference = "Continue"

# Load Token
$token = (Get-Content gitee-cli\.env | Select-String "GITEE_TOKEN=").ToString().Split("=")[1].Trim()
$env:GITEE_TOKEN = $token

$OWNER = "fourthz"
$REPO = "gitee-tools-test"
$CLI = "target\debug\gitee.exe"

$REPORT = "TEST_REPORT_FINAL_CLEAN.md"
" # Gitee-RS Final Comprehensive Test Report`n" | Out-File -FilePath $REPORT
"Generated on: $(Get-Date)`n" | Out-File -Append -FilePath $REPORT
"| Category | Function | Status | Result/Output |" | Out-File -Append -FilePath $REPORT
"| --- | --- | --- | --- |" | Out-File -Append -FilePath $REPORT

function Run-Test($cat, $name, $cmd) {
    Write-Host "Testing $name..."
    $output = & $CLI @cmd 2>&1 | Out-String
    $status = if ($LASTEXITCODE -eq 0) { "✅ Pass" } else { "❌ Fail" }
    $cleanOutput = $output.Replace("`n", " ").Replace("|", "/").Trim()
    if ($cleanOutput.Length -gt 200) { $cleanOutput = $cleanOutput.Substring(0, 200) + "..." }
    "| $cat | $name | $status | $cleanOutput |" | Out-File -Append -FilePath $REPORT
}

# --- 1. Basic Info ---
Run-Test "Repo" "Get Info" @("repo", "info", $OWNER, $REPO)
Run-Test "Repo" "Star" @("repo-ext", "star", $OWNER, $REPO)

# --- 2. Labels ---
Run-Test "Labels" "Create Label" @("labels", "create", $OWNER, $REPO, "v0.9", "00FF00")
Run-Test "Labels" "List Labels" @("labels", "list", $OWNER, $REPO)
Run-Test "Labels" "Delete Label" @("labels", "delete", $OWNER, $REPO, "v0.9")

# --- 3. Milestones ---
Run-Test "Milestones" "List Milestones" @("issues-ext", "milestone-list", $OWNER, $REPO)

# --- 4. Issues ---
Run-Test "Issues" "Create Issue" @("issues", "create", $OWNER, $REPO, "Issue after Refactor", "--body", "Checking refactored structure")
Run-Test "Issues" "List Issues" @("issues", "list", $OWNER, $REPO)

# --- 5. Wiki (The heavy lifter) ---
Run-Test "Wiki" "Create Final-Test" @("wiki", "create", $OWNER, $REPO, "Final-Test", "Structure cleanup successful at $(Get-Date)")
Run-Test "Wiki" "List Wiki" @("wiki", "list", $OWNER, $REPO)
Run-Test "Wiki" "Get Final-Test" @("wiki", "get", $OWNER, $REPO, "Final-Test")

# --- 6. Files ---
Run-Test "Files" "List Files" @("files", "list", $OWNER, $REPO)

Write-Host "Refactoring complete. Full test finished. Report: $REPORT"
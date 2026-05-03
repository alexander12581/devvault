#!/bin/bash
# devvault pre-commit hook
# Scans staged files for hardcoded secrets before commit

echo "Running devvault secret scan on staged files..."

# Get list of staged files
staged_files=$(git diff --cached --name-only --diff-filter=ACM)

if [ -z "$staged_files" ]; then
    echo "No staged files to scan."
    exit 0
fi

# Run devvault scan on staged files
scan_result=$(devvault scan --staged 2>&1)
exit_code=$?

if [ $exit_code -ne 0 ]; then
    echo "Error running devvault scan:"
    echo "$scan_result"
    exit 1
fi

if echo "$scan_result" | grep -q "Potential secrets found"; then
    echo "WARNING: Potential secrets found in staged files!"
    echo "$scan_result"
    echo ""
    echo "Please review the above findings and remove any hardcoded secrets."
    echo "Use 'devvault set KEY=VALUE' to store secrets securely."
    echo ""
    echo "To bypass this check (not recommended), use: git commit --no-verify"
    exit 1
fi

echo "No secrets found. Proceeding with commit."
exit 0
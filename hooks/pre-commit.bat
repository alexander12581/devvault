@echo off
REM devvault pre-commit hook
REM Scans staged files for hardcoded secrets before commit

echo Running devvault secret scan on staged files...

REM Get list of staged files
for /f "tokens=*" %%f in ('git diff --cached --name-only --diff-filter=ACM') do (
    set staged_files=%%f
)

if "%staged_files%"=="" (
    echo No staged files to scan.
    exit /b 0
)

REM Run devvault scan on staged files
devvault scan --staged > scan_result.txt 2>&1
set exit_code=%errorlevel%

if %exit_code% neq 0 (
    echo Error running devvault scan:
    type scan_result.txt
    del scan_result.txt
    exit /b 1
)

findstr /C:"Potential secrets found" scan_result.txt >nul
if %errorlevel% equ 0 (
    echo WARNING: Potential secrets found in staged files!
    type scan_result.txt
    echo.
    echo Please review the above findings and remove any hardcoded secrets.
    echo Use 'devvault set KEY=VALUE' to store secrets securely.
    echo.
    echo To bypass this check (not recommended), use: git commit --no-verify
    del scan_result.txt
    exit /b 1
)

echo No secrets found. Proceeding with commit.
del scan_result.txt
exit /b 0
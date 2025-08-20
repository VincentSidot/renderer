@echo off
echo Running pre-commit checks...
echo.

echo Checking code formatting...
cargo fmt -- --check
if %errorlevel% neq 0 (
    echo Error: Code is not properly formatted. Run 'cargo fmt' to fix.
    exit /b 1
)

echo.
echo Checking for clippy warnings...
cargo clippy -- -D warnings
if %errorlevel% neq 0 (
    echo Error: Clippy found warnings. Please fix them before committing.
    exit /b 1
)

echo.
echo Running tests...
cargo test -q
if %errorlevel% neq 0 (
    echo Error: Tests failed. Please fix them before committing.
    exit /b 1
)

echo.
echo All checks passed!
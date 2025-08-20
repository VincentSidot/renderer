# Git Hooks

This project uses Git hooks to ensure code quality and consistency.

## Pre-commit Hook

The pre-commit hook automatically runs the following checks before allowing a commit:

1. Code formatting with `cargo fmt`
2. Clippy linting with `cargo clippy`
3. Unit tests with `cargo test`

If any of these checks fail, the commit will be aborted.

## Installation

The hooks are automatically installed when you set up the project. If you need to reinstall them, run:

```bash
copy scripts\pre-commit .git\hooks\pre-commit
```

## Customization

You can modify the hooks by editing the files in the `scripts` directory. After making changes, you'll need to copy them to the `.git\hooks` directory to take effect.
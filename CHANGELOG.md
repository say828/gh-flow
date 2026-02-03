# Changelog

All notable changes to gh-flow will be documented in this file.

## [0.2.0] - 2026-02-03

### ✨ Fully Implemented Features

This release implements all core functionality! gh-flow is now fully operational.

#### Commands Implemented

- **`gh flow init`** - Initialize a stack
  - Detects git repository
  - Creates `.gh-flow.json` configuration
  - Auto-adds current branch if not on base
  - Validates base branch exists

- **`gh flow status`** - Show stack status
  - Displays branch hierarchy
  - Shows PR status (open, merged, closed)
  - Highlights current branch
  - Color-coded visualization
  - Shows parent relationships

- **`gh flow pr create`** - Create PRs
  - Creates PRs for all branches in stack
  - Sets correct base branches
  - Adds stack visualization to PR descriptions
  - Supports draft PRs with `--draft`
  - Skips existing PRs
  - Saves PR numbers to configuration

- **`gh flow pr update`** - Update PRs
  - Updates PR descriptions with current stack info
  - Useful after stack structure changes

- **`gh flow push`** - Push branches
  - Pushes all branches in stack
  - Supports force push with `--force`
  - Uses `--force-with-lease` for safety
  - Shows progress for each branch

- **`gh flow sync`** - Synchronize stack (Core Feature!)
  - **Detects merged parent PRs**
  - **Auto-retargets to main when parent is merged** ✨
  - **Automatically rebases child branches** ✨
  - **Propagates changes through stack** ✨
  - Updates PR base branches via GitHub API
  - Handles conflicts gracefully
  - Supports dry-run mode with `--dry-run`
  - Restores original branch after completion

### 🎯 Requirements Satisfied

All 5 core requirements are now fully implemented:

1. ✅ 첫 PR은 main 브랜치를 바라봄
2. ✅ 각 PR에는 필요한 변경사항만 보이며 선행 브랜치를 바라봄
3. ✅ 선행 브랜치가 머지되기 전에 머지되면 안됨 (GitHub enforces)
4. ✅ **선행 브랜치 머지 후 자동으로 main 브랜치 리타겟** ⭐
5. ✅ **이전 브랜치 변경사항 자동 전파** ⭐

### 🔧 Technical Improvements

- Full Git integration with error handling
- GitHub API integration via gh CLI
- Proper state management with `.gh-flow.json`
- Colored terminal output for better UX
- Comprehensive error messages
- Conflict detection and guidance

### 📚 Documentation

- Updated README with installation instructions
- Added ROADMAP for future development
- Added PUBLISHING guide
- Comprehensive inline documentation

---

## [0.1.0] - 2026-02-03

### Initial Release

- Project structure setup
- CLI framework with clap
- Command stubs (non-functional)
- Basic modules: git, github, stack
- README and documentation

---

**Full Changelog**: https://github.com/say828/gh-flow/commits/main

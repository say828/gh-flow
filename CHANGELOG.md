# Changelog

All notable changes to gh-flow will be documented in this file.

## [0.3.0] - 2026-02-03

### 🚀 Major Release: Developer Experience & Advanced Features

This release brings significant improvements to developer experience and adds advanced automation features.

#### 🔧 Developer Experience

- **CI/CD Pipeline** (GitHub Actions)
  - Automated testing on every PR (Ubuntu, macOS, Windows)
  - Code formatting check with rustfmt
  - Linting with clippy
  - Automated releases on version tags

- **Binary Releases** (5 platforms)
  - macOS Intel (x86_64-apple-darwin)
  - macOS Apple Silicon (aarch64-apple-darwin)
  - Linux x64 (x86_64-unknown-linux-gnu)
  - Linux ARM64 (aarch64-unknown-linux-gnu)
  - Windows x64 (x86_64-pc-windows-msvc)

- **Shell Completions** (`gh flow completions`)
  - bash
  - zsh
  - fish
  - powershell
  - elvish

- **Homebrew Formula**
  - Template for `say828/homebrew-tap`
  - Automated formula update script

#### ✨ New Commands

- **`gh flow merge`** - Merge PRs with auto-merge support
  - `--auto`: Enable auto-merge when CI passes
  - `--wait-ci`: Wait for CI to pass before merging
  - Checks review approval status
  - Sequential processing to maintain stack order

- **`gh flow split`** - Split large stacks
  - `-i, --interactive`: Select branches interactively
  - Automatic midpoint split in non-interactive mode
  - Retargets split branches to base

- **`gh flow completions`** - Generate shell completions
  - `gh flow completions bash > ~/.bash_completion.d/gh-flow`
  - `gh flow completions zsh > ~/.zsh/completions/_gh-flow`

#### 🎨 UX Improvements

- **Progress Indicators**
  - Animated spinners for long operations
  - Progress bars for multi-branch operations
  - Clear success/failure indicators (✓/✗/⚠)

- **Validation Framework**
  - Git repository validation
  - GitHub CLI authentication checks
  - Uncommitted changes warnings

- **Better Error Messages**
  - Context-aware suggestions
  - Recovery instructions
  - Colored output

#### 🔄 Enhanced Commands

- **`gh flow sync --wait-ci`**
  - Check CI status before syncing each branch
  - Skip branches with pending/failed CI

#### 📁 New Files

```
.github/
  workflows/
    ci.yml           # CI pipeline
    release.yml      # Release automation
homebrew/
  gh-flow.rb         # Homebrew formula template
scripts/
  update-homebrew.sh # Formula update script
src/
  progress.rs        # Progress utilities
  validation.rs      # Validation checks
  commands/
    merge.rs         # Auto-merge workflow
    split.rs         # Stack splitting
    completions.rs   # Shell completions
```

#### 📦 Dependencies Added

- `clap_complete = "4.5"` - Shell completions
- `indicatif = "0.17"` - Progress indicators
- `dialoguer = "0.11"` - Interactive prompts
- `console = "0.15"` - Terminal utilities

---

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

# gh-flow

GitHub CLI extension for managing stacked PRs (Pull Requests).

## 🎯 Core Requirements

This tool was built to satisfy the following workflow requirements:

1. **첫 PR은 main 브랜치를 바라봄**
   The first PR in the stack targets the main branch

2. **이후 각 PR에는 필요한 변경사항만 보이며 선행 브랜치를 바라봄**
   Each subsequent PR shows only its changes and targets the previous branch

3. **선행 브랜치가 머지되기 전에 머지되면 안됨**
   Child PRs cannot be merged before their parent PRs (enforced by GitHub)

4. **선행 브랜치가 머지되면 자동으로 다음 브랜치가 main 브랜치를 바라봐야 함**
   When a parent PR is merged, the next PR automatically retargets to main

5. **이전 브랜치에 변경사항이 생기면 이후 브랜치에 자동으로 전파되어야 함**
   Changes in earlier branches automatically propagate to later branches in the stack

## 🚀 What is gh-flow?

gh-flow helps you manage **stacked PRs** - a workflow where you break large features into smaller, sequential pull requests. Each PR builds on top of the previous one, making code reviews easier and faster.

### Key Features

- **Auto-discover**: Automatically detects branch chains from git history
- **Automatic Stacking**: Each PR targets its parent branch, showing only relevant changes
- **Smart Sync**: Automatically rebase and retarget PRs when parent branches are merged
- **Change Propagation**: Updates automatically cascade through your stack
- **Merge Order Enforcement**: GitHub prevents merging child PRs before parents
- **Clean Visualization**: See your entire PR stack at a glance
- **PR Templates**: Customize PR descriptions with templates

## 📦 Installation

### Prerequisites

- [GitHub CLI (`gh`)](https://cli.github.com/) - Required

### Option 1: gh extension (Recommended)

```bash
# Install via gh CLI extension system
gh extension install say828/gh-flow

# Upgrade to latest version
gh extension upgrade say828/gh-flow

# Verify installation
gh flow --version
```

### Option 2: Download Binary

Download the latest release from [GitHub Releases](https://github.com/say828/gh-flow/releases):

```bash
# macOS (Apple Silicon)
curl -L https://github.com/say828/gh-flow/releases/latest/download/gh-flow-aarch64-apple-darwin.tar.gz | tar xz
mv gh-flow ~/.local/bin/

# macOS (Intel)
curl -L https://github.com/say828/gh-flow/releases/latest/download/gh-flow-x86_64-apple-darwin.tar.gz | tar xz
mv gh-flow ~/.local/bin/

# Linux (x86_64)
curl -L https://github.com/say828/gh-flow/releases/latest/download/gh-flow-x86_64-unknown-linux-gnu.tar.gz | tar xz
mv gh-flow ~/.local/bin/

# Linux (ARM64)
curl -L https://github.com/say828/gh-flow/releases/latest/download/gh-flow-aarch64-unknown-linux-gnu.tar.gz | tar xz
mv gh-flow ~/.local/bin/
```

### Option 3: Build from Source

```bash
# Clone the repository
git clone https://github.com/say828/gh-flow.git
cd gh-flow

# Build release binary
cargo build --release

# Install to local bin
mkdir -p ~/.local/bin
cp target/release/gh-flow ~/.local/bin/
chmod +x ~/.local/bin/gh-flow

# Add to PATH (if not already)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc  # or ~/.bashrc
source ~/.zshrc
```

### Verify Installation

```bash
gh-flow --version
gh-flow --help
```

## 🚀 Quick Start

### 1. Initialize a Stack

```bash
# In your git repository with existing branches
gh flow init

# Or specify a custom base branch
gh flow init --base develop
```

This will:
- Auto-discover branch chains from git history
- Save configuration to `~/.config/gh-flow/repos/<owner>/<repo>/gh-flow.json`
- Create a PR template at `~/.config/gh-flow/pr-template.md`

### 2. Create PRs

```bash
# Create PRs for all branches without existing PRs
gh flow pr create

# Or create as drafts
gh flow pr create --draft
```

This automatically:
- Skips branches that already have PRs
- Creates PRs with correct base branches
- Adds stack visualization to PR descriptions

### 3. Keep Stack in Sync

```bash
# After making changes to earlier PRs
gh flow sync

# Preview changes without executing
gh flow sync --dry-run
```

## 📖 Commands

### `gh flow init`

Initialize a new PR stack in the current repository.

```bash
gh flow init [--base <branch>]
```

**Options:**
- `-b, --base <branch>` - Base branch (default: `main`)

**What it does:**
- Discovers branch chains from git history (no manual setup needed!)
- Creates configuration file
- Creates PR template if not exists

### `gh flow status`

Show the current state of your PR stack.

```bash
gh flow status
```

Displays:
- Stack structure (branch hierarchy)
- PR status for each branch
- Current branch indicator

### `gh flow sync`

Synchronize the entire stack by rebasing and retargeting PRs.

```bash
gh flow sync [--dry-run]
```

**Options:**
- `-d, --dry-run` - Show what would be done without executing

### `gh flow push`

Push all branches in the stack to remote.

```bash
gh flow push [--force]
```

**Options:**
- `-f, --force` - Force push with `--force-with-lease`

### `gh flow pr create`

Create PRs for all branches in the stack.

```bash
gh flow pr create [--draft]
```

**Options:**
- `-d, --draft` - Create PRs as drafts

### `gh flow pr update`

Update existing PRs (descriptions, bases, etc.).

```bash
gh flow pr update
```

## 📁 Configuration

Configuration is stored following XDG Base Directory specification:

```
~/.config/gh-flow/
├── pr-template.md                     # Global PR template
└── repos/
    └── <owner>/<repo>/
        ├── gh-flow.json               # Stack configuration
        └── pr-template.md             # Repo-specific PR template (optional)
```

### PR Template

Customize PR descriptions by editing `~/.config/gh-flow/pr-template.md`:

```markdown
## Summary

{{stack}}

## Test Plan
- [ ] Tests pass

---
*Generated by gh-flow*
```

**Template Variables:**
- `{{stack}}` - Stack visualization
- `{{branch}}` - Current branch name

**Template Priority:**
1. Repo-specific template (`repos/<owner>/<repo>/pr-template.md`)
2. Global template (`pr-template.md`)
3. Default template

## 🔄 Workflow Example

### Scenario: Adding a new feature with 3 PRs

```bash
# 1. Create your branches as usual
git checkout main
git checkout -b feat/db-schema
# ... make changes ...
git commit -m "Add user table schema"

git checkout -b feat/api-endpoints
# ... make changes ...
git commit -m "Add user CRUD endpoints"

git checkout -b feat/user-ui
# ... make changes ...
git commit -m "Add user management UI"

# 2. Initialize and create PRs
gh flow init
gh flow pr create

# Result:
# PR #1: main ← feat/db-schema
# PR #2: feat/db-schema ← feat/api-endpoints
# PR #3: feat/api-endpoints ← feat/user-ui
```

### When PR #1 is Merged

```bash
# Run sync to update the stack
gh flow sync

# Result:
# ✓ PR #1: Merged
# PR #2: main ← feat/api-endpoints (retargeted!)
# PR #3: feat/api-endpoints ← feat/user-ui
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

MIT License

## 🙏 Acknowledgments

Inspired by:
- [gh-stack](https://github.com/timothyandrew/gh-stack)
- [Graphite](https://graphite.dev)
- [ghstack](https://github.com/ezyang/ghstack)

---

Built with Rust

# Publishing gh-flow as a GitHub CLI Extension

## Current Status

✅ Repository created: https://github.com/say828/gh-flow
✅ Code pushed to GitHub
✅ README updated with installation instructions

## Installation Options

### Option 1: Via gh extension install (Requires build from source)

Users can install using:

```bash
gh extension install say828/gh-flow
```

**Note**: This will automatically run `install.sh` which builds the Rust binary.

### Option 2: Manual installation

```bash
git clone https://github.com/say828/gh-flow.git
cd gh-flow
./install.sh
```

## Future: Binary Releases

For the best user experience, consider adding pre-compiled binaries via GitHub Releases:

1. **Set up GitHub Actions CI/CD**
   - Build for multiple platforms (macOS, Linux, Windows)
   - Create release artifacts
   - Automate release process

2. **Use cargo-dist or similar tools**
   ```bash
   cargo install cargo-dist
   cargo dist init
   ```

3. **Update installation to download binaries**
   - Faster installation (no compilation needed)
   - Better for users without Rust

## How gh Extensions Work

When a user runs `gh extension install say828/gh-flow`:

1. gh CLI clones the repository
2. Looks for an executable script (`install.sh` or precompiled binary)
3. Runs the installation script
4. Makes `gh flow` command available

## Testing Installation

```bash
# Uninstall if already installed
gh extension remove flow

# Install from your repo
gh extension install say828/gh-flow

# Test
gh flow --version
gh flow --help
```

## Publishing Checklist

- [x] Create GitHub repository
- [x] Push source code
- [x] Add installation script (install.sh)
- [x] Update README with installation instructions
- [ ] Test installation via `gh extension install`
- [ ] Create GitHub Release with binaries (optional, recommended)
- [ ] Add CI/CD for automated releases (optional)
- [ ] Publish to gh extension marketplace (optional)

## Next Steps

1. Test the extension installation:
   ```bash
   gh extension install say828/gh-flow
   ```

2. If it works, you're done! Share the installation command:
   ```bash
   gh extension install say828/gh-flow
   ```

3. For better UX, consider adding precompiled binaries in GitHub Releases

## Resources

- [gh extension docs](https://docs.github.com/en/github-cli/github-cli/creating-github-cli-extensions)
- [cargo-dist](https://opensource.axo.dev/cargo-dist/)
- [GitHub Actions for Rust](https://github.com/actions-rs/meta)

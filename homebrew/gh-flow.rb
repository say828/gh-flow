class GhFlow < Formula
  desc "GitHub CLI extension for managing stacked PRs"
  homepage "https://github.com/say828/gh-flow"
  version "VERSION_PLACEHOLDER"
  license "MIT"

  on_macos do
    on_intel do
      url "https://github.com/say828/gh-flow/releases/download/v#{version}/gh-flow-x86_64-apple-darwin.tar.gz"
      sha256 "SHA256_PLACEHOLDER_MACOS_INTEL"
    end
    on_arm do
      url "https://github.com/say828/gh-flow/releases/download/v#{version}/gh-flow-aarch64-apple-darwin.tar.gz"
      sha256 "SHA256_PLACEHOLDER_MACOS_ARM"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/say828/gh-flow/releases/download/v#{version}/gh-flow-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "SHA256_PLACEHOLDER_LINUX_INTEL"
    end
    on_arm do
      url "https://github.com/say828/gh-flow/releases/download/v#{version}/gh-flow-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "SHA256_PLACEHOLDER_LINUX_ARM"
    end
  end

  def install
    bin.install "gh-flow"

    # Install shell completions
    generate_completions_from_executable(bin/"gh-flow", "completions")
  end

  test do
    assert_match "gh-flow", shell_output("#{bin}/gh-flow --version")
  end
end

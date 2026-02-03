class Ocli < Formula
  desc "🦉 Ollama Command Line Interface - AI coding assistant with LCARS styling"
  homepage "https://github.com/wiseowltechnet/ollama-ocli"
  url "https://github.com/wiseowltechnet/ollama-ocli/archive/v0.2.0.tar.gz"
  sha256 "PLACEHOLDER_SHA256"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release"
    bin.install "target/release/ocli"
  end

  test do
    assert_match "OCLI v0.2.0", shell_output("#{bin}/ocli --version 2>&1", 0)
  end
end

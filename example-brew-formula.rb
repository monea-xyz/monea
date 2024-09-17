class MoneaCli < Formula
  desc "Monea CLI tool"
  homepage "https://github.com/monea-xyz/monea-core"
  url "https://github.com/monea-xyz/monea-cli/releases/download/v1.0.0/monea-cli.tar.gz"
  sha256 ""
  version "0.1.0"

  def install
    bin.install "monea-cli"
    bin.install "kurtosis"
    prefix.install "engine"
  end

  test do
    system "#{bin}/monea-cli", "--version"
  end
end
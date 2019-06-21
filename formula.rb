class Grrs < Formula
  desc "..."
  homepage "https://github.com/tacoda/grrs"
  url "https://github.com/tacoda/grrs/archive/v0.1.0.tar.gz"
  version "1.0.0"
  # sha256 "e17405adc655824dec3ca6e2a9a4b199a715743ed5f0948df58f6bb369267aa3"

  def install
    bin.install "target/release/grrs"
    # prefix.install Dir["_completions"]
    # prefix.install Dir["_helpers"]
    # prefix.install Dir["_config"]
  end
end

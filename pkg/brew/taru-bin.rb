# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
# PLEASE REMOVE ALL GENERATED COMMENTS BEFORE SUBMITTING YOUR PULL REQUEST!
class TaruBin < Formula
  version "v0.2.0"
  desc "Taru is a local-first workflow-runner for unix-based systems"
  homepage "https://github.com/rasmusNygren/taru"
  url "https://github.com/RasmusNygren/taru/releases/download/#{version}/taru-#{version}-aarch64-apple-darwin.tar.gz"
  sha256 "18364048b1157b05547bd264f5d296ae657384f42d10bddf84e7f25d3d15bfcb"
  license "MIT"

  depends_on "rust" => :build

  def install
    bin.install "taru"
  end
end

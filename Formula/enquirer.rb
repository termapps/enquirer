class Enquirer < Formula
  version "0.1.0"
  desc "Command Line Utility for Stylish Interactive Prompts"
  homepage "https://github.com/termapps/enquirer"

  if OS.mac?
    url "https://github.com/termapps/enquirer/releases/download/v#{version}/enquirer-v#{version}-x86_64-apple-darwin.zip"
    sha256 "7f260a6ca70fe27f8d20a56cc31b0462b32a63c5acbf65059ed07b2b31f622c3" # mac
  elsif OS.linux?
    url "https://github.com/termapps/enquirer/releases/download/v#{version}/enquirer-v#{version}-x86_64-unknown-linux-gnu.zip"
    sha256 "c90bffe2b0feb56402009f3c87d05c6e0d9cb0c5f8e9052d4661627dac00bbf5" # linux
  end

  def install
    bin.install "enquirer"
    man1.install "enquirer.1"
  end

  test do
    system "#{bin}/enquirer", "--version"
  end
end

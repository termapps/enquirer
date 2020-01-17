class Enquirer < Formula
  version "0.0.1"
  desc "Command Line Utility for Stylish Interactive Prompts"
  homepage "https://github.com/termapps/enquirer"

  if OS.mac?
    url "https://github.com/termapps/enquirer/releases/download/#{version}/enquirer-#{version}-x86_64-apple-darwin.zip"
    sha256 "c700385a3a470fcc137e2bc71d8c1a2b3ff5723a7da0cb081859b18036076d35" # mac
  elsif OS.linux?
    url "https://github.com/termapps/enquirer/releases/download/#{version}/enquirer-#{version}-x86_64-unknown-linux-gnu.zip"
    sha256 "2c900c315a059dbd3079f5be781ef674bf0c124d1254cc986ba1c127ae4fa406" # linux
  end

  def install
    bin.install "enquirer"
    man1.install "enquirer.1"
  end

  test do
    system "#{bin}/enquirer", "--version"
  end
end

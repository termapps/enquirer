<h1 align="center">enquirer</h1>

<p align="center">
	<a href="https://travis-ci.org/termapps/enquirer">
		<img src="https://img.shields.io/travis/termapps/enquirer.svg" alt="travis">
	</a>
</p>

<p align="center">
	<b>Command Line Utility for Stylish Interactive Prompts</b>
	<br>
	<sub>(like fzf but for all types)</sub>
	<br>
	<sub>(uses <a href="https://github.com/mitsuhiko/dialoguer">dialoguer</a> underneath)</sub>
</p>

## Getting started

Get started with Enquirer, the most powerful command line utility for creating interactive CLI prompts.

* [Install](#install)
* [Usage](#usage)
* [Prompts](#prompts)
* [About](#about)

## Install

`enquirer` is available on Linux, macOS

### With [Homebrew](https://brew.sh/)

```
$ brew tap termapps/tap https://github.com/termapps/enquirer
$ brew install enquirer
```

This is recommended way for installation on macOS since updating to the new version is easy.

### With [cargo](https://crates.io/)

```
$ cargo install enquirer
```

### Direct

Pre-built binary executables are available at [releases page](https://github.com/termapps/enquirer/releases) for macOS (64bit), Linux (64bit, 32bit).

Download and unarchive the binary then put the executable in `$PATH`.

## Usage

### Command Line Utility

The main reason I created this tool is to use it as an stylish interactive and user-friendly prompt for bash scripting.

```bash
#!/bin/bash

TRUE="true"
CONFIRM=$(enquirer confirm -m "Do you want to continue?" -d)

if [ "$TRUE" = "$CONFIRM" ]; then
    echo "Continuing ..."
else
    echo "Thanks for using this tool. Quitting ..."
    exit
fi
```

See [prompts](#prompts) for more information on subcommands.

```
enquirer 0.1.0
Command Line Utility for Stylish Interactive Prompts

USAGE:
    enquirer [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help        Prints help information
        --no-color    Disable colors in the prompt
    -V, --version     Prints version information

SUBCOMMANDS:
    confirm    Prompt that returns `true` or `false`
    help       Prints this message or the help of the given subcommand(s)
```

### Library

If you want the dialoguer theme used in this tool you can add this package to your `Cargo.toml`

```rust
use dialoguer::Confirmation;
use enquirer::ColoredTheme;

fn main() {
    let prompt = Confirmation::with_theme(&ColoredTheme::default())
        .with_text("Do you want to continue?")
        .with_default(true);

    if prompt.interact()? {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }
}
```

## Prompts

* [Confirm Prompt](#confirm-prompt)

### Confirm Prompt

Prompt that returns `true` or `false` (as strings).

<p align="center">
	<img src="media/confirm.svg" alt="Enquirer Confirm Prompt" width="750">
</p>

#### Usage

```
enquirer 0.1.0
Prompt that returns `true` or `false`

USAGE:
    enquirer confirm [FLAGS] [OPTIONS] --message <message>

FLAGS:
    -d, --default    Default value for the prompt is `true`
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --message <message>      Message for the prompt
```

## About

### Changelog

Please see [CHANGELOG.md](CHANGELOG.md).

### License
MIT/X11

### Bug Reports
Report [here](http://github.com/termapps/enquirer/issues).

### Creator
Pavan Kumar Sunkara (pavan.sss1991@gmail.com)

Follow me on [github](https://github.com/users/follow?target=pksunkara), [twitter](http://twitter.com/pksunkara)

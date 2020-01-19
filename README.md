<h1 align="center">enquirer</h1>

<p align="center">
  <a href="https://travis-ci.org/termapps/enquirer">
    <img alt="Travis" src="https://img.shields.io/travis/termapps/enquirer?style=flat-square" alt="travis">
  </a>
  <a href="https://termapps.zulipchat.com/#narrow/stream/220422-enquirer">
    <img alt="Zulip" src="https://img.shields.io/badge/zulip-join_chat-brightgreen.svg?style=flat-square">
  </a>
  <a href="https://crates.io/crates/enquirer">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/enquirer?style=flat-square">
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
$ brew tap termapps/tap
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

> Check [roadmap](#roadmap) for other ways

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
enquirer 0.3.0
Command Line Utility for Stylish Interactive Prompts

USAGE:
    enquirer [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help        Prints help information
        --no-color    Disable colors in the prompt
    -V, --version     Prints version information

SUBCOMMANDS:
    confirm         Prompt that returns `true` or `false` (as strings)
    help            Prints this message or the help of the given subcommand(s)
    input           Prompt that takes user input and returns a string
    multi-select    Prompt that allows the user to select multiple items from a list of options
    secret          Prompt that takes user input, hides it from the terminal, and returns a string
    select          Prompt that allows the user to select from a list of options
    sort            Prompt that allows the user to sort items in a list
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
* [Input Prompt](#input-prompt)
* [Secret Prompt](#secret-prompt)
* [Select Prompt](#select-prompt)
* [Multi Select Prompt](#multi-select-prompt)
* [Sort Prompt](#sort-prompt)

### Confirm Prompt

Prompt that returns `true` or `false` (as strings)

<p align="center">
  <img src="media/confirm.svg" alt="Enquirer Confirm Prompt" width="750">
</p>

#### Usage

```
enquirer-confirm 0.3.0
Prompt that returns `true` or `false` (as strings)

USAGE:
    enquirer confirm [FLAGS] [OPTIONS] --message <message>

FLAGS:
    -d, --default    Default value for the prompt is `true`
    -h, --help       Prints help information

OPTIONS:
    -m, --message <message>      Message for the prompt
```

### Input Prompt

Prompt that takes user input and returns a string

<p align="center">
  <img src="media/input.svg" alt="Enquirer Input Prompt" width="750">
</p>

#### Usage

```
enquirer-input 0.3.0
Prompt that takes user input and returns a string

USAGE:
    enquirer input [FLAGS] [OPTIONS] --message <message>

FLAGS:
    -a, --allow-empty    Allow empty input. Conflicts with `default`
    -h, --help           Prints help information

OPTIONS:
    -d, --default <default>    Default value for the prompt
    -m, --message <message>    Message for the prompt
```

### Secret Prompt

Prompt that takes user input, hides it from the terminal, and returns a string

<p align="center">
  <img src="media/secret.svg" alt="Enquirer Secret Prompt" width="750">
</p>

#### Usage

```
enquirer-secret 0.3.0
Prompt that takes user input, hides it from the terminal, and returns a string

USAGE:
    enquirer secret [FLAGS] [OPTIONS] --message <message>

FLAGS:
    -a, --allow-empty    Allow empty secret
    -h, --help           Prints help information

OPTIONS:
    -c, --confirm <confirm>    Enable confirmation prompt with this message
    -e, --error <error>        Error message when secrets doesn't match during confirmation
    -m, --message <message>    Message for the prompt
```

### Select Prompt

Prompt that allows the user to select from a list of options

<p align="center">
  <img src="media/select.svg" alt="Enquirer Select Prompt" width="750">
</p>

#### Usage

```
enquirer-select 0.3.0
Prompt that allows the user to select from a list of options

USAGE:
    enquirer select [FLAGS] [OPTIONS] --message <message> [items]...

FLAGS:
    -h, --help     Prints help information
    -p, --paged    Enables paging. Uses your terminal size

OPTIONS:
    -m, --message <message>      Message for the prompt
    -s, --selected <selected>    Specify number of the item that will be selected by default

ARGS:
    <items>...    Items that can be selected
```

### Multi Select Prompt

Prompt that allows the user to select multiple items from a list of options

<p align="center">
  <img src="media/multi-select.svg" alt="Enquirer Multi Select Prompt" width="750">
</p>

#### Usage

```
enquirer-multi-select 0.3.0
Prompt that allows the user to select multiple items from a list of options

USAGE:
    enquirer multi-select [FLAGS] [OPTIONS] --message <message> [--] [items]...

FLAGS:
    -h, --help         Prints help information
        --no-inline    Do not print the selected items on the prompt line
    -p, --paged        Enables paging. Uses your terminal size

OPTIONS:
    -m, --message <message>         Message for the prompt
    -s, --selected <selected>...    Specify numbers of items that will be selected by default

ARGS:
    <items>...    Items that can be selected
```

### Sort Prompt

Prompt that allows the user to sort items in a list

<p align="center">
  <img src="media/sort.svg" alt="Enquirer Sort Prompt" width="750">
</p>

#### Usage

```
enquirer-sort 0.3.0
Prompt that allows the user to sort items in a list

USAGE:
    enquirer sort [FLAGS] --message <message> [items]...

FLAGS:
    -h, --help         Prints help information
        --no-inline    Do not print the sorted items on the prompt line
    -p, --paged        Enables paging. Uses your terminal size

OPTIONS:
    -m, --message <message>    Message for the prompt

ARGS:
    <items>...    Items that can be sorted
```

## About

### Roadmap

* [Man page generation](https://github.com/clap-rs/clap/issues/552)
* Packaging for other operating systems
* [Auto Complete prompt](https://github.com/enquirer/enquirer#autocomplete-prompt) (like fzf)
* [Snippet Prompt](https://github.com/enquirer/enquirer#snippet-prompt)
* [Export hooks for Clap](https://github.com/clap-rs/clap/issues/1471)

#### Dialoguer issues

* Select and Multi-Select prompts cursors are after the items list while waiting for user. Maybe change them to be on prompt line.

### Changelog

Please see [CHANGELOG.md](CHANGELOG.md).

### License
MIT/X11

### Bug Reports
Report [here](http://github.com/termapps/enquirer/issues).

### Creator
Pavan Kumar Sunkara (pavan.sss1991@gmail.com)

Follow me on [github](https://github.com/users/follow?target=pksunkara), [twitter](http://twitter.com/pksunkara)

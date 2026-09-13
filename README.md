You can use this repository to generate a new Rails application without having to install Ruby on your machine.

It works by using Docker to generate the Rails application for you. Docker takes care of installing the right Ruby and
Rails versions for you, so you don't have to worry about it.

## Prerequisites

You need to have Docker installed on your machine. You can find instructions on how to install Docker on your machine
[here](https://docs.docker.com/engine/install/).

**On Linux**, your user needs to be a member of the docker group. rails-new does not work with sudo. You can find instructions on this [here](https://docs.docker.com/engine/install/linux-postinstall/)

## Installation

Go to the [latest release](https://github.com/rails/rails-new/releases/latest) and download the executable for your platform (not the source code). For example, on M1 macOS this would be `rails-new-aarch64-apple-darwin.tar.gz`. Once the download is complete, unzip the `.tar.gz` file, which will create the `rails-new` executable. Move the executable into your path so that it is ready to run from the command line.

## Usage

To generate a new Rails application, you can run the following command:

```bash
rails-new myapp
```

Or with options:
```bash
rails-new myapp --main
```

The first time you attempt to use the executable in macOS you may see a message like this:

> "rails-new" can’t be opened because Apple cannot check it for malicious software

In that case, please go to System Settings → Privacy & Security. You'll see a section mentioning "rails-new" with a button labeled "Allow Anyway" that you have to click.

The list of available options is found in the [Rails guides](https://guides.rubyonrails.org/command_line.html#rails-new).

## Configuration

You can set default Ruby and Rails versions in `~/.config/rails-new/config.toml`:

```toml
ruby_version = "3.3.0"
rails_version = "8.0.0"
rails_args = [
  "--database=postgresql",
  "--skip-test",
  "--skip-system-test",
]
```

If `XDG_CONFIG_HOME` is set, the configuration file is read from
`$XDG_CONFIG_HOME/rails-new/config.toml` instead. On Windows, `%APPDATA%\rails-new\config.toml`
is used when `XDG_CONFIG_HOME` is not set.

Command-line options take precedence over values in the configuration file:

```bash
rails-new --ruby-version 3.4.0 myapp
```

Rails arguments from the command line are appended after `rails_args`, allowing command-line
options to override configurable options that Rails accepts more than once:

```bash
rails-new myapp --database=sqlite3
```

If no Ruby version is specified in either location, `latest` is used. If no Rails version is
specified, the latest Rails version is installed.

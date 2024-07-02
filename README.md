You can use this repository to generate a new Rails application without having to install Ruby on your machine.

It works by using Docker to generate the Rails application for you. Docker takes care of installing the right Ruby and
Rails versions for you, so you don't have to worry about it.

## Prerequisites

You need to have Docker installed on your machine. You can find instructions on how to install Docker on your machine
[here](https://docs.docker.com/engine/install/).

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

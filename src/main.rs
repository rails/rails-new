// Write a CLI program that call the bash file rails-new inside the bin folder.

// use std::process::Command;
mod rails_new;
use rails_new::Cli;
use std::io::Write;
use std::process::{Command, Stdio};

use clap::Parser;

fn main() {
    let cli = Cli::parse();

    // read the content of the DOCKERFILE and store it in a variable
    let dockerfile = include_str!("../Dockerfile");

    let ruby_version = cli.ruby_version.unwrap();
    let rails_version = cli.rails_version.unwrap();

    // Run docker build --build-arg RUBY_VERSION=$RUBY_VERSION --build-arg RAILS_VERSION=$RAILS_VERSION -t rails-new-$RUBY_VERSION-$RAILS_VERSION
    // passing the content of DOCKERFILE to the command stdin
    let mut child = Command::new("docker")
        .arg("build")
        .arg("--build-arg")
        .arg(format!("RUBY_VERSION={}", ruby_version))
        .arg("--build-arg")
        .arg(format!("RAILS_VERSION={}", rails_version))
        .arg("-t")
        .arg(format!("rails-new-{}-{}", ruby_version, rails_version))
        .arg("-")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to execute process");

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(dockerfile.as_bytes()).unwrap();

    let status = child.wait().expect("failed to wait on child");

    assert!(status.success());

    // Run the image with docker run -v $(pwd):/$(pwd) -w $(pwd) rails-new-$RUBY_VERSION-$RAILS_VERSION rails new $@
    let binding = std::env::current_dir().unwrap();
    let current_dir = binding.to_str().unwrap();

    let status = Command::new("docker")
        .arg("run")
        .arg("-v")
        .arg(format!("{}:{}", current_dir, current_dir))
        .arg("-w")
        .arg(current_dir)
        .arg(format!("rails-new-{}-{}", ruby_version, rails_version))
        .arg("rails")
        .arg("new")
        .args(cli.args)
        .status()
        .expect("Failed to execute process");

    assert!(status.success());
}

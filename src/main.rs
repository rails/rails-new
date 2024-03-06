// Write a CLI program that call the bash file rails-new inside the bin folder.

// use std::process::Command;
mod rails_new;
use rails_new::Cli;
use std::process::Command;

use clap::Parser;

fn main() {
    let cli = Cli::parse();

    // Execute the bash file rails-new
    let status = Command::new("bash")
        .arg("bin/rails-new")
        .arg(cli.name)
        .status()
        .expect("Failed to execute process");

    assert!(status.success());
}

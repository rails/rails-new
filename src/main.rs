// Write a CLI program that call the bash file rails-new inside the bin folder.

use std::process::Command;

fn main() {
    // Read the application name from the command arguments
    let args: Vec<String> = std::env::args().collect();
    let app_name = &args[1];

    // Execute the bash file rails-new
    let status = Command::new("bash")
        .arg("bin/rails-new")
        .arg(app_name)
        .status()
        .expect("Failed to execute process");

    assert!(status.success());
}

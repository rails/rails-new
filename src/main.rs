// Write a CLI program that call the bash file rails-new inside the bin folder.

// use std::process::Command;
mod docker_client;
mod rails_new;
use rails_new::Cli;
use std::io::Write;

use clap::Parser;

use crate::docker_client::DockerClient;

fn main() {
    let cli = Cli::parse();

    let dockerfile: &[u8];
    let user_id: Option<u32>;
    let group_id: Option<u32>;

    if cfg!(windows) {
        user_id = None;
        group_id = None;
        dockerfile = include_bytes!("../Dockerfile.windows");
    } else {
        user_id = Some(users::get_current_uid());
        group_id = Some(users::get_current_gid());
        dockerfile = include_bytes!("../Dockerfile");
    }

    let ruby_version = cli.ruby_version;
    let rails_version = cli.rails_version;

    // Run docker build --build-arg RUBY_VERSION=$RUBY_VERSION --build-arg RAILS_VERSION=$RAILS_VERSION -t rails-new-$RUBY_VERSION-$RAILS_VERSION
    // passing the content of DOCKERFILE to the command stdin
    let mut child = DockerClient::build_image(&ruby_version, &rails_version, user_id, group_id)
        .spawn()
        .expect("Failed to execute process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin.write_all(dockerfile).unwrap();
    });

    let status = child.wait().expect("failed to wait on child");

    assert!(status.success());

    // Run the image with docker run -v $(pwd):/$(pwd) -w $(pwd) rails-new-$RUBY_VERSION-$RAILS_VERSION rails new $@
    let status = DockerClient::run_image(&ruby_version, &rails_version, cli.args)
        .status()
        .expect("Failed to execute process");

    assert!(status.success());
}

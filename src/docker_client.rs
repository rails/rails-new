use std::process::{Command, Stdio};

pub struct DockerClient {}

impl DockerClient {
    pub fn build_image(
        ruby_version: &str,
        rails_version: &str,
        user_id: Option<u32>,
        group_id: Option<u32>,
    ) -> Command {
        let mut command = Command::new("docker");

        command
            .arg("build")
            .arg("--build-arg")
            .arg(format!("RUBY_VERSION={}", ruby_version))
            .arg("--build-arg")
            .arg(format!("RAILS_VERSION={}", rails_version));

        user_id.map(|id| command.args(["--build-arg", &format!("USER_ID={}", id)]));
        group_id.map(|id| command.args(["--build-arg", &format!("GROUP_ID={}", id)]));

        command
            .arg("-t")
            .arg(format!("rails-new-{}-{}", ruby_version, rails_version))
            .arg("-")
            .stdin(Stdio::piped());

        command
    }

    pub fn run_image(ruby_version: &str, rails_version: &str, args: Vec<String>) -> Command {
        let binding = std::env::current_dir().unwrap();
        let current_dir = binding.to_str().unwrap();

        let mut command = Command::new("docker");

        command
            .arg("run")
            .arg("--rm")
            .arg("-v")
            .arg(format!("{}:{}", current_dir, current_dir))
            .arg("-w")
            .arg(current_dir)
            .arg(format!("rails-new-{}-{}", ruby_version, rails_version))
            .arg("rails")
            .arg("new")
            .args(args);

        command
    }
}

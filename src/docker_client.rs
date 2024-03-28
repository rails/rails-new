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

    pub fn get_help(ruby_version: &str, rails_version: &str) -> Command {
        let mut command = Command::new("docker");

        command
            .arg("run")
            .arg("--rm")
            .arg(format!("rails-new-{}-{}", ruby_version, rails_version))
            .arg("rails")
            .arg("new")
            .arg("--help");

        command
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env::current_dir, ffi::OsStr};

    #[test]
    fn build_image() {
        let command = DockerClient::build_image("3.2.3", "7.1.3", None, None);

        assert_eq!(command.get_program(), "docker");

        let args: Vec<&OsStr> = command.get_args().collect();

        assert_eq!(
            args,
            &[
                "build",
                "--build-arg",
                "RUBY_VERSION=3.2.3",
                "--build-arg",
                "RAILS_VERSION=7.1.3",
                "-t",
                "rails-new-3.2.3-7.1.3",
                "-",
            ]
        );
    }

    #[test]
    fn build_image_with_user_id() {
        let command = DockerClient::build_image("3.2.3", "7.1.3", Some(1000), None);

        assert_eq!(command.get_program(), "docker");

        let args: Vec<&OsStr> = command.get_args().collect();

        assert_eq!(
            args,
            &[
                "build",
                "--build-arg",
                "RUBY_VERSION=3.2.3",
                "--build-arg",
                "RAILS_VERSION=7.1.3",
                "--build-arg",
                "USER_ID=1000",
                "-t",
                "rails-new-3.2.3-7.1.3",
                "-",
            ]
        );
    }

    #[test]
    fn build_image_with_group_id() {
        let command = DockerClient::build_image("3.2.3", "7.1.3", None, Some(1000));

        assert_eq!(command.get_program(), "docker");

        let args: Vec<&OsStr> = command.get_args().collect();

        assert_eq!(
            args,
            &[
                "build",
                "--build-arg",
                "RUBY_VERSION=3.2.3",
                "--build-arg",
                "RAILS_VERSION=7.1.3",
                "--build-arg",
                "GROUP_ID=1000",
                "-t",
                "rails-new-3.2.3-7.1.3",
                "-",
            ]
        );
    }

    #[test]
    fn run_image() {
        let command = DockerClient::run_image("3.2.3", "7.1.3", vec!["my_app".to_string()]);

        assert_eq!(command.get_program(), "docker");

        let binding = current_dir().unwrap();
        let current_dir = binding.to_str().unwrap();

        let args: Vec<&OsStr> = command.get_args().collect();

        assert_eq!(
            args,
            &[
                "run",
                "--rm",
                "-v",
                &format!("{}:{}", current_dir, current_dir),
                "-w",
                current_dir,
                "rails-new-3.2.3-7.1.3",
                "rails",
                "new",
                "my_app",
            ]
        );
    }

    #[test]
    fn get_help() {
        let command = DockerClient::get_help("3.2.3", "7.1.3");

        assert_eq!(command.get_program(), "docker");

        let args: Vec<&OsStr> = command.get_args().collect();

        assert_eq!(
            args,
            &[
                "run",
                "--rm",
                "rails-new-3.2.3-7.1.3",
                "rails",
                "new",
                "--help",
            ]
        );
    }
}

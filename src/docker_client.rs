use std::process::{Command, Stdio};

pub struct DockerClient {}

impl DockerClient {
    pub fn build_image(
        ruby_version: &str,
        maybe_rails_version: Option<&str>,
        user_id: Option<u32>,
        group_id: Option<u32>,
        rebuild: bool,
    ) -> Command {
        let mut command = Command::new("docker");

        command.arg("build");

        if rebuild {
            command.arg("--no-cache");
        }

        Self::set_build_arg(&mut command, "RUBY_VERSION", ruby_version);
        if let Some(rails_version) = maybe_rails_version {
            Self::set_build_arg(&mut command, "RAILS_VERSION", rails_version);
        }

        if let Some(id) = user_id {
            Self::set_build_arg(&mut command, "USER_ID", &id.to_string())
        }
        if let Some(id) = group_id {
            Self::set_build_arg(&mut command, "GROUP_ID", &id.to_string())
        }

        command.arg("-t");

        Self::set_image_name(&mut command, ruby_version, maybe_rails_version);

        command.arg("-").stdin(Stdio::piped());

        command
    }

    pub fn run_image(
        ruby_version: &str,
        rails_version: Option<&str>,
        args: Vec<String>,
    ) -> Command {
        let mut command = Self::run();

        Self::set_workdir(&mut command);
        Self::set_image_name(&mut command, ruby_version, rails_version);
        Self::set_rails_new(&mut command, args);

        command
    }

    pub fn get_help(ruby_version: &str, rails_version: Option<&str>) -> Command {
        let mut command = Self::run();

        Self::set_image_name(&mut command, ruby_version, rails_version);
        Self::set_rails_new(&mut command, vec!["--help".to_string()]);

        command
    }

    fn run() -> Command {
        let mut command = Command::new("docker");

        command.args(["run", "--rm"]);

        command
    }

    fn set_build_arg(command: &mut Command, key: &str, value: &str) {
        command.args(["--build-arg", &format!("{}={}", key, value)]);
    }

    fn set_workdir(command: &mut Command) {
        let path = std::env::current_dir().expect("Failed to get current directory");
        let absolute_path = canonicalize_os_path(&path).expect("Failed to build directory");
        let current_dir = absolute_path
            .to_str()
            .expect("Failed to get current directory");

        command
            .arg("-v")
            .arg(format!("{}:{}", current_dir, current_dir))
            .args(["-w", current_dir]);
    }

    fn set_image_name(
        command: &mut Command,
        ruby_version: &str,
        maybe_rails_version: Option<&str>,
    ) {
        if let Some(rails_version) = maybe_rails_version {
            command.arg(format!("rails-new-{}-{}", ruby_version, rails_version));
        } else {
            command.arg(format!("rails-new-{}", ruby_version));
        }
    }

    fn set_rails_new(command: &mut Command, args: Vec<String>) {
        command.args(["rails", "new"]).args(args);
    }
}

fn canonicalize_os_path(path: &std::path::Path) -> std::io::Result<std::path::PathBuf> {
    let canonicalized = std::fs::canonicalize(path)?;

    if cfg!(windows) {
        let path_str = canonicalized.to_str().unwrap();
        // On Windows only, check if the path starts with the UNC prefix
        // example:  \\?\C:\path\to\file
        if path_str.starts_with(r"\\?\") {
            // drop UNC prefix
            let path_str = &path_str[4..];
            // grab the drive letter
            let drive_letter = &path_str[0..1];
            // swap \ for /
            let rest_of_path = &path_str[2..].replace(r"\", "/");
            // rebuild as /C/path/to/file
            return Ok(std::path::PathBuf::from(format!(
                "/{}/{}",
                drive_letter, rest_of_path
            )));
        }
    }
    Ok(canonicalized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env::current_dir, ffi::OsStr};

    #[test]
    fn build_image() {
        let command = DockerClient::build_image("3.2.3", Some("7.1.3"), None, None, false);

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
        let command = DockerClient::build_image("3.2.3", Some("7.1.3"), Some(1000), None, false);

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
        let command = DockerClient::build_image("3.2.3", Some("7.1.3"), None, Some(1000), false);

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
    fn build_image_with_rebuild_flag() {
        let command = DockerClient::build_image("3.2.3", Some("7.1.3"), None, None, true);

        let args: Vec<&OsStr> = command.get_args().collect();

        assert_eq!(
            args,
            &[
                "build",
                "--no-cache",
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
    fn build_image_without_rails_version() {
        let command = DockerClient::build_image("3.2.3", None, None, None, false);

        let args: Vec<&OsStr> = command.get_args().collect();

        assert_eq!(
            args,
            &[
                "build",
                "--build-arg",
                "RUBY_VERSION=3.2.3",
                "-t",
                "rails-new-3.2.3",
                "-",
            ]
        );
    }

    #[test]
    fn build_image_with_both_ids() {
        let command = DockerClient::build_image("3.2.3", Some("7.1.3"), Some(1000), Some(1000), false);

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
        let command = DockerClient::run_image("3.2.3", Some("7.1.3"), vec!["my_app".to_string()]);

        assert_eq!(command.get_program(), "docker");

        let binding = current_dir().unwrap();
        let absolute_path = canonicalize_os_path(&binding).unwrap();
        let current_dir = absolute_path.to_str().unwrap();

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
    fn run_image_without_rails_version() {
        let command = DockerClient::run_image("3.2.3", None, vec!["my_app".to_string()]);

        let binding = current_dir().unwrap();
        let absolute_path = canonicalize_os_path(&binding).unwrap();
        let current_dir = absolute_path.to_str().unwrap();

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
                "rails-new-3.2.3",
                "rails",
                "new",
                "my_app",
            ]
        );
    }

    #[test]
    fn get_help() {
        let command = DockerClient::get_help("3.2.3", Some("7.1.3"));

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

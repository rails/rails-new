use serde::Deserialize;
use std::{env, fs, path::Path, path::PathBuf};

const DEFAULT_RUBY_VERSION: &str = "latest";

#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    ruby_version: Option<String>,
    rails_version: Option<String>,
    #[serde(default)]
    rails_args: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let Some(path) = config_path() else {
            return Ok(Self::default());
        };

        Self::load_from(&path)
    }

    pub fn resolve_versions(
        &self,
        ruby_version: Option<String>,
        rails_version: Option<String>,
    ) -> (String, Option<String>) {
        (
            ruby_version
                .or_else(|| self.ruby_version.clone())
                .unwrap_or_else(|| DEFAULT_RUBY_VERSION.to_string()),
            rails_version.or_else(|| self.rails_version.clone()),
        )
    }

    pub fn merge_rails_args(&self, mut command_line_args: Vec<String>) -> Vec<String> {
        if command_line_args.is_empty() {
            return command_line_args;
        }

        let command_line_options = command_line_args.split_off(1);
        command_line_args.extend(self.rails_args.iter().cloned());
        command_line_args.extend(command_line_options);
        command_line_args
    }

    fn load_from(path: &Path) -> Result<Self, String> {
        let contents = match fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::default())
            }
            Err(error) => {
                return Err(format!(
                    "Failed to read config file {}: {error}",
                    path.display()
                ))
            }
        };

        toml::from_str(&contents)
            .map_err(|error| format!("Failed to parse config file {}: {error}", path.display()))
    }
}

fn config_path() -> Option<PathBuf> {
    if let Some(config_home) = non_empty_env("XDG_CONFIG_HOME") {
        return Some(config_home.join("rails-new/config.toml"));
    }

    #[cfg(windows)]
    if let Some(app_data) = non_empty_env("APPDATA") {
        return Some(app_data.join("rails-new/config.toml"));
    }

    if let Some(home) = non_empty_env("HOME") {
        return Some(home.join(".config/rails-new/config.toml"));
    }

    #[cfg(windows)]
    if let Some(user_profile) = non_empty_env("USERPROFILE") {
        return Some(user_profile.join(".config/rails-new/config.toml"));
    }

    None
}

fn non_empty_env(name: &str) -> Option<PathBuf> {
    env::var_os(name)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        sync::atomic::{AtomicUsize, Ordering},
        time::{SystemTime, UNIX_EPOCH},
    };

    static TEST_FILE_ID: AtomicUsize = AtomicUsize::new(0);

    fn config_file(contents: &str) -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let id = TEST_FILE_ID.fetch_add(1, Ordering::Relaxed);
        let path = env::temp_dir().join(format!(
            "rails-new-config-{}-{timestamp}-{id}.toml",
            std::process::id()
        ));
        fs::write(&path, contents).unwrap();
        path
    }

    #[test]
    fn missing_config_uses_defaults() {
        let path = env::temp_dir().join(format!(
            "rails-new-missing-config-{}-{}.toml",
            std::process::id(),
            TEST_FILE_ID.fetch_add(1, Ordering::Relaxed)
        ));

        let config = Config::load_from(&path).unwrap();
        let versions = config.resolve_versions(None, None);

        assert_eq!(versions, ("latest".to_string(), None));
    }

    #[test]
    fn reads_versions_from_config() {
        let path = config_file(
            r#"
ruby_version = "3.3.0"
rails_version = "8.0.0"
rails_args = ["--database=postgresql", "--skip-test"]
"#,
        );

        let config = Config::load_from(&path).unwrap();
        fs::remove_file(path).unwrap();

        assert_eq!(config.ruby_version, Some("3.3.0".to_string()));
        assert_eq!(config.rails_version, Some("8.0.0".to_string()));
        assert_eq!(config.rails_args, ["--database=postgresql", "--skip-test"]);
    }

    #[test]
    fn command_line_versions_override_config() {
        let config = Config {
            ruby_version: Some("3.2.0".to_string()),
            rails_version: Some("7.2.0".to_string()),
            rails_args: Vec::new(),
        };

        assert_eq!(
            config.resolve_versions(Some("3.3.0".to_string()), Some("8.0.0".to_string())),
            ("3.3.0".to_string(), Some("8.0.0".to_string()))
        );
    }

    #[test]
    fn config_rails_args_are_inserted_before_command_line_options() {
        let config = Config {
            ruby_version: None,
            rails_version: None,
            rails_args: vec![
                "--database=postgresql".to_string(),
                "--skip-test".to_string(),
            ],
        };

        let args = config.merge_rails_args(vec![
            "my_app".to_string(),
            "--database=sqlite3".to_string(),
            "--skip-system-test".to_string(),
        ]);

        assert_eq!(
            args,
            [
                "my_app",
                "--database=postgresql",
                "--skip-test",
                "--database=sqlite3",
                "--skip-system-test",
            ]
        );
    }

    #[test]
    fn rejects_unknown_fields() {
        let path = config_file("ruby_verison = \"3.3.0\"");

        let error = Config::load_from(&path).unwrap_err();
        fs::remove_file(path).unwrap();

        assert!(error.contains("unknown field `ruby_verison`"));
    }
}

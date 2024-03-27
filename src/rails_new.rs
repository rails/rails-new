use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None, subcommand_negates_reqs = true)]
pub struct Cli {
    #[clap(trailing_var_arg = true, required = true)]
    /// arguments passed to `rails new`
    pub args: Vec<String>,
    #[clap(long, short = 'u', default_value = "3.2.3")]
    pub ruby_version: String,
    #[clap(long, short = 'r', default_value = "7.1.3")]
    pub rails_version: String,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Prints `rails new --help`
    RailsHelp {},
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;

        Cli::command().debug_assert()
    }

    #[test]
    fn arguments_are_directed_to_rails_new() -> Result<(), Box<dyn std::error::Error>> {
        use clap::CommandFactory;

        let m = Cli::command().get_matches_from(vec!["rails-new", "my_app", "--main"]);

        let trail: Vec<_> = m.get_many::<String>("args").unwrap().collect();

        assert_eq!(trail, &["my_app", "--main"]);

        Ok(())
    }
}

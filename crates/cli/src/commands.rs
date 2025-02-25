use clap::{Parser, Subcommand};
use query_ai_core::logging::LogLevel;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(short, long, value_name = "FILE")]
    pub config: Option<String>,

    #[clap(short, long, value_name = "LEVEL", default_value = "info")]
    pub log_level: String,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[clap(name = "index")]
    Index(IndexCommand),

    #[clap(name = "update")]
    Update(UpdateCommand),

    #[clap(name = "query")]
    Query(QueryCommand),
}

#[derive(Debug, Parser)]
pub struct IndexCommand {
    #[clap(value_name = "REPO_URL")]
    pub repo_url: String,

    #[clap(short, long, value_name = "BRANCH", default_value = "main")]
    pub branch: String,

    #[clap(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Parser)]
pub struct UpdateCommand {
    #[clap(value_name = "PAYLOAD_FILE")]
    pub payload_file: String,

    #[clap(short, long)]
    pub force: bool,
}

#[derive(Debug, Parser)]
pub struct QueryCommand {
    #[clap(value_name = "QUERY")]
    pub query: String,

    #[clap(short, long, value_name = "NUM", default_value = "10")]
    pub limit: usize,

    #[clap(short, long, value_name = "FORMAT", default_value = "text")]
    pub format: String,
}

impl Cli {
    pub fn get_log_level(&self) -> Result<LogLevel, query_ai_core::Error> {
        self.log_level.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use query_ai_core::logging::LogLevel;

    #[test]
    fn test_cli_parse_default_log_level() {
        let args = vec!["query_ai", "query", "my query"];
        let cli = Cli::parse_from(args);

        assert_eq!(cli.log_level, "info");
        let log_level = cli.get_log_level().unwrap();
        assert!(matches!(log_level, LogLevel::Info));
    }

    #[test]
    fn test_cli_parse_custom_log_level() {
        let args = vec!["query_ai", "--log-level", "debug", "query", "my query"];
        let cli = Cli::parse_from(args);

        assert_eq!(cli.log_level, "debug");
        let log_level = cli.get_log_level().unwrap();
        assert!(matches!(log_level, LogLevel::Debug));
    }

    #[test]
    fn test_cli_parse_invalid_log_level() {
        let args = vec!["query_ai", "--log-level", "invalid", "query", "my query"];
        let cli = Cli::parse_from(args);

        assert_eq!(cli.log_level, "invalid");
        assert!(cli.get_log_level().is_err());
    }

    #[test]
    fn test_cli_parse_index_command() {
        let args = vec!["query_ai", "index", "https://github.com/example/repo.git"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Command::Index(cmd) => {
                assert_eq!(cmd.repo_url, "https://github.com/example/repo.git");
                assert_eq!(cmd.branch, "main");
                assert!(!cmd.verbose);
            }
            _ => panic!("Expected Index command"),
        }
    }

    #[test]
    fn test_cli_parse_index_command_with_options() {
        let args = vec![
            "query_ai",
            "index",
            "https://github.com/example/repo.git",
            "--branch",
            "develop",
            "--verbose",
        ];
        let cli = Cli::parse_from(args);

        match cli.command {
            Command::Index(cmd) => {
                assert_eq!(cmd.repo_url, "https://github.com/example/repo.git");
                assert_eq!(cmd.branch, "develop");
                assert!(cmd.verbose);
            }
            _ => panic!("Expected Index command"),
        }
    }

    #[test]
    fn test_cli_parse_update_command() {
        let args = vec!["query_ai", "update", "webhook.json"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Command::Update(cmd) => {
                assert_eq!(cmd.payload_file, "webhook.json");
                assert!(!cmd.force);
            }
            _ => panic!("Expected Update command"),
        }
    }

    #[test]
    fn test_cli_parse_update_command_with_force() {
        let args = vec!["query_ai", "update", "webhook.json", "--force"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Command::Update(cmd) => {
                assert_eq!(cmd.payload_file, "webhook.json");
                assert!(cmd.force);
            }
            _ => panic!("Expected Update command"),
        }
    }

    #[test]
    fn test_cli_parse_query_command() {
        let args = vec!["query_ai", "query", "how does error handling work?"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Command::Query(cmd) => {
                assert_eq!(cmd.query, "how does error handling work?");
                assert_eq!(cmd.limit, 10);
                assert_eq!(cmd.format, "text");
            }
            _ => panic!("Expected Query command"),
        }
    }

    #[test]
    fn test_cli_parse_query_command_with_options() {
        let args = vec![
            "query_ai",
            "query",
            "how does error handling work?",
            "--limit",
            "20",
            "--format",
            "json",
        ];
        let cli = Cli::parse_from(args);

        match cli.command {
            Command::Query(cmd) => {
                assert_eq!(cmd.query, "how does error handling work?");
                assert_eq!(cmd.limit, 20);
                assert_eq!(cmd.format, "json");
            }
            _ => panic!("Expected Query command"),
        }
    }

    #[test]
    fn test_cli_parse_config_option() {
        let args = vec!["query_ai", "--config", "my_config.json", "query", "test"];
        let cli = Cli::parse_from(args);

        assert_eq!(cli.config, Some("my_config.json".to_string()));
    }
}

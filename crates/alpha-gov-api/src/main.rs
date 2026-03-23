use std::path::PathBuf;
use std::process;

use alpha_gov_api_core::{
    AppConfig, OutputFormat,
    config::{config_display, default_config_path, get_credential, set_credential},
    output::print_json,
};
use clap::{Parser, Subcommand};

/// Government APIs as structured JSON for agentic AI consumption.
#[derive(Debug, Parser)]
#[command(name = "alpha-gov-api", version, about)]
pub struct Cli {
    /// Output as pretty-printed JSON instead of compact.
    #[arg(long, global = true)]
    pub pretty: bool,

    /// Suppress all output except the JSON result.
    #[arg(long, short, global = true)]
    pub quiet: bool,

    /// Path to configuration file.
    #[arg(long, value_name = "PATH", global = true)]
    pub config: Option<PathBuf>,

    /// Configuration profile to use.
    #[arg(long, value_name = "NAME", global = true)]
    pub profile: Option<String>,

    /// Use sandbox/test endpoints instead of production.
    #[arg(long, global = true)]
    pub sandbox: bool,

    /// Validate the request without executing it.
    #[arg(long, global = true)]
    pub dry_run: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Manage configuration and credentials.
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Debug, Subcommand)]
pub enum ConfigAction {
    /// Display current configuration (credentials are redacted).
    Show,
    /// Set a configuration or credential value.
    Set {
        /// The key to set (e.g., "ch.api_key", "sandbox").
        key: String,
        /// The value to set.
        value: String,
        /// Store in config file instead of OS keychain.
        #[arg(long)]
        plaintext: bool,
    },
    /// Get a credential value.
    Get {
        /// The credential key (e.g., "ch.api_key").
        key: String,
    },
    /// Show the path to the configuration file.
    Path,
}

impl Cli {
    pub fn output_format(&self) -> OutputFormat {
        if self.pretty {
            OutputFormat::Pretty
        } else {
            OutputFormat::Json
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialise tracing (off by default; controlled by RUST_LOG env var).
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .try_init();

    let cli = Cli::parse();

    if let Err(e) = run(&cli) {
        if !cli.quiet {
            eprintln!("error: {e}");
        }
        process::exit(1);
    }
}

fn run(cli: &Cli) -> alpha_gov_api_core::Result<()> {
    let command = match &cli.command {
        Some(cmd) => cmd,
        None => {
            eprintln!("alpha-gov-api: no command specified. Run with --help for usage.");
            return Ok(());
        }
    };

    match command {
        Command::Config { action } => handle_config(cli, action),
    }
}

fn handle_config(cli: &Cli, action: &ConfigAction) -> alpha_gov_api_core::Result<()> {
    let format = cli.output_format();

    match action {
        ConfigAction::Show => {
            let config = AppConfig::load(cli.config.as_deref(), cli.profile.as_deref())?;
            let display = config_display(&config);
            print_json(&display, format)
                .map_err(|e| alpha_gov_api_core::ConfigError::Output { source: e })?;
            Ok(())
        }
        ConfigAction::Set {
            key,
            value,
            plaintext,
        } => {
            let mut config = AppConfig::load(cli.config.as_deref(), cli.profile.as_deref())?;
            set_credential(&mut config, key, value, *plaintext)?;
            if !cli.quiet {
                eprintln!("Set {key}");
            }
            Ok(())
        }
        ConfigAction::Get { key } => {
            let config = AppConfig::load(cli.config.as_deref(), cli.profile.as_deref())?;
            match get_credential(&config, key)? {
                Some(val) => {
                    print_json(&serde_json::json!({ "key": key, "value": val }), format)
                        .map_err(|e| alpha_gov_api_core::ConfigError::Output { source: e })?;
                }
                None => {
                    if !cli.quiet {
                        eprintln!("No value set for {key}");
                    }
                }
            }
            Ok(())
        }
        ConfigAction::Path => {
            let path = match &cli.config {
                Some(p) => p.clone(),
                None => default_config_path()?,
            };
            println!("{}", path.display());
            Ok(())
        }
    }
}

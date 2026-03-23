use std::path::PathBuf;

use alpha_gov_api_core::OutputFormat;
use clap::Parser;

/// Government APIs as structured JSON for agentic AI consumption.
#[derive(Debug, Parser)]
#[command(name = "alpha-gov-api", version, about)]
pub struct Cli {
    /// Output as pretty-printed JSON instead of compact.
    #[arg(long)]
    pub pretty: bool,

    /// Suppress all output except the JSON result.
    #[arg(long, short)]
    pub quiet: bool,

    /// Path to configuration file.
    #[arg(long, value_name = "PATH")]
    pub config: Option<PathBuf>,

    /// Configuration profile to use.
    #[arg(long, value_name = "NAME")]
    pub profile: Option<String>,

    /// Use sandbox/test endpoints instead of production.
    #[arg(long)]
    pub sandbox: bool,

    /// Validate the request without executing it.
    #[arg(long)]
    pub dry_run: bool,
}

impl Cli {
    /// Resolve the output format from CLI flags.
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
    let _cli = Cli::parse();

    // No subcommands yet — clap will show help automatically when invoked
    // with no arguments. With only global flags provided, we print a hint.
    eprintln!("alpha-gov-api: no command specified. Run with --help for usage.");
}

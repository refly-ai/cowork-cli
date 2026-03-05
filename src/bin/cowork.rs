use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use cowork::commands::clone::{
    run_contribute, run_init, run_metadata, run_preview, run_resource, run_update, run_version,
};
use cowork::commands::self_update::run as run_self_update;
use tracing_subscriber::{prelude::*, EnvFilter};

#[derive(Debug, Parser)]
#[command(name = "cowork", version, about = "Cowork CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long = "log-level", default_value = "warn")]
    log_level: String,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Update cowork via install.sh
    SelfUpdate(SelfUpdateArgs),
    /// Manage local clone workspace
    Clone(CloneArgs),
}

#[derive(Debug, clap::Args)]
struct SelfUpdateArgs {
    /// Target version (example: 1.2.3)
    #[arg(long)]
    version: Option<String>,
}

#[derive(Debug, clap::Args)]
struct CloneArgs {
    #[command(subcommand)]
    command: CloneCommands,
}

#[derive(Debug, Subcommand)]
enum CloneCommands {
    /// Compare local and remote clone package versions
    Version,
    /// Clone configured repository into COWORK_HOME
    Init,
    /// Pull latest changes with --ff-only
    Update,
    /// Show clone metadata and tree
    Metadata,
    /// Start preview in tmux session
    Preview,
    /// Print worktree-based contribution flow
    Contribute,
    /// Print undefined resource convention guide
    Resource,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    init_logging(&cli.log_level)?;

    match cli.command {
        Commands::SelfUpdate(args) => run_self_update(args.version.as_deref()),
        Commands::Clone(args) => match args.command {
            CloneCommands::Version => run_version(),
            CloneCommands::Init => run_init(),
            CloneCommands::Update => run_update(),
            CloneCommands::Metadata => run_metadata(),
            CloneCommands::Preview => run_preview(),
            CloneCommands::Contribute => run_contribute(),
            CloneCommands::Resource => run_resource(),
        },
    }
}

fn init_logging(level: &str) -> Result<()> {
    let env_filter = EnvFilter::builder()
        .parse(level)
        .with_context(|| format!("invalid log level: {}", level))?;

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .try_init()
        .context("failed to initialize logger")?;

    Ok(())
}

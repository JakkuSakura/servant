use clap::{Parser, Subcommand};
use common_libs::*;
use servant::project::{list_projects, load_project};
use std::path::{Path, PathBuf};

#[derive(Subcommand)]
pub enum Commands {
    /// build a project or service
    Build {
        project: String,
        service: Option<String>,
    },
    /// package a project or service
    Package {
        project: String,
        service: Option<String>,
    },
    /// deploy a project or service
    Deploy {
        project: String,
        service: Option<String>,
    },
    /// start a project or service
    Start {
        project: String,
        service: Option<String>,
    },
    /// stop a project or service
    Run {
        project: String,
        service: Option<String>,
    },
    /// reload a project or service
    Stop {
        project: String,
        service: Option<String>,
    },
    /// enable a project or service
    Reload {
        project: String,
        service: Option<String>,
    },
    /// disable a project or service
    Enable {
        project: String,
        service: Option<String>,
    },
    /// status a project or service
    Disable {
        project: String,
        service: Option<String>,
    },
    /// config a project or service
    Status {
        project: String,
        service: Option<String>,
    },
    /// list a project or service
    Config {
        project: String,
        service: Option<String>,
    },
    /// list all projects or service within the project
    List { project: Option<String> },
}
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// directory where the config files are located
    #[clap(short, long, default_value = "etc")]
    config: PathBuf,
}

pub async fn print_projects(config: PathBuf, project: Option<String>) -> Result<()> {
    match project {
        Some(project) => {
            let project = load_project(&config.join(Path::new(&project)))?;
            project.print_project()?;
        }
        None => {
            let projects = list_projects(&config)?;
            for p in projects {
                p.print_project()?;
            }
        }
    }
    Ok(())
}
#[tokio::main]
async fn main() -> Result<()> {
    setup_logs(LogLevel::Info)?;
    let cli = Cli::parse();
    let config = cli.config;
    match cli.command {
        Commands::List { project } => print_projects(config, project).await?,
        _ => bail!("subcommand not supported yet"),
    }
    Ok(())
}

use clap::{Parser, Subcommand};

mod bag;
mod provider;
mod ui;
mod fs_utils;
mod commands;
mod installer;
mod error;

#[derive(Parser)]
#[command(name = "grain")]
#[command(about = "Godot Rust Addons Installation Ninja")]
#[command(long_about = "A CLI tool for managing Rust addons in Godot projects")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Godot Rust addon project
    Init {
        /// Target directory (defaults to current directory)
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Search query
    Search {
        #[arg()]
        term: String,
    },
    /// Install a Rust addon from a repository
    Install {
        /// Repository URL or addon name
        #[arg()]
        addon: String,
        /// Specific version to install
        #[arg(short, long)]
        version: Option<String>,
    },
    /// Restore all addons from bag.yaml
    Restore {

    },
    /// List installed addons
    List {
        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,
    },
    /// Remove an installed addon
    Remove {
        /// Name of the addon to remove
        #[arg()]
        name: String,
        /// Force removal without confirmation
        #[arg(short, long)]
        force: bool,
    },
    /// Update installed addons
    Update {
        /// Update all addons
        #[arg(short, long)]
        all: bool,
        /// Specific addon to update
        #[arg()]
        addon: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init { path } => {
            commands::handle_init(path).await
        }
        Commands::Install { addon, version } => {
            commands::handle_install(addon, version).await
        }
        Commands::Search { term } => {
            commands::handle_search(term).await
        }
        Commands::Restore { } => {
            commands::handle_restore().await
        }
        Commands::List { verbose } => {
            commands::handle_list(verbose).await
        }
        Commands::Remove { name, force } => {
            commands::handle_remove(name, force).await
        }
        Commands::Update { all, addon } => {
            commands::handle_update(all, addon).await
        }
    };

    if let Err(e) = result {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
}



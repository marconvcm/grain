use clap::{Parser, Subcommand};

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
        /// Name of the addon
        #[arg(short, long)]
        name: String,
        /// Target directory (defaults to current directory)
        #[arg(short, long)]
        path: Option<String>,
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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, path } => {
            let target_path = path.unwrap_or_else(|| ".".to_string());
            println!(
                "Initializing new Godot Rust addon '{}' in '{}'",
                name, target_path
            );
            println!("This would create the basic structure for a Godot Rust addon.");
            // TODO: Implement addon initialization logic
        }
        Commands::Install { addon, version } => {
            match version {
                Some(v) => println!("Installing addon '{}' version '{}'", addon, v),
                None => println!("Installing latest version of addon '{}'", addon),
            }
            println!("This would download and install the specified addon.");
            // TODO: Implement addon installation logic
        }
        Commands::List { verbose } => {
            println!("Listing installed addons:");
            if verbose {
                println!("Verbose mode - showing detailed information");
            }
            println!("No addons currently installed.");
            // TODO: Implement addon listing logic
        }
        Commands::Remove { name, force } => {
            if force {
                println!("Force removing addon '{}'", name);
            } else {
                println!("Removing addon '{}' (with confirmation)", name);
            }
            println!("This would remove the specified addon.");
            // TODO: Implement addon removal logic
        }
        Commands::Update { all, addon } => {
            if all {
                println!("Updating all installed addons");
            } else if let Some(addon_name) = addon {
                println!("Updating addon '{}'", addon_name);
            } else {
                println!("Please specify either --all or an addon name to update");
                std::process::exit(1);
            }
            println!("This would update the specified addon(s).");
            // TODO: Implement addon update logic
        }
    }
}

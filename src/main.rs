use clap::{Parser, Subcommand};
use std::{
    env,
    fs,
    path::PathBuf,
    process::Command,
};

#[derive(Parser)]
#[command(author, version, about = "Toggle Starship presets in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Toggle between two presets (e.g., minimal & verbose)
    Toggle,
    /// List all presets in ~/.config/starship_presets
    List,
    /// Set a specific preset by name
    Set {
        preset_name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // Directory with our Starship config .toml files
    let home = env::var("HOME").unwrap();
    let preset_dir = format!("{}/.config/starship_presets", home);
    let minimal = format!("{}/minimal.toml", &preset_dir);
    let verbose = format!("{}/verbose.toml", &preset_dir);

    // Current setting (or empty if not set)
    let current_config = env::var("STARSHIP_CONFIG").unwrap_or_default();

    match cli.command {
        Some(Commands::Toggle) => {
            if current_config == minimal {
                println!("Switching to Verbose Prompt...");
                set_config(&verbose);
            } else {
                println!("Switching to Minimal Prompt...");
                set_config(&minimal);
            }
        }
        Some(Commands::List) => {
            println!("Available presets in {preset_dir}:");
            if let Ok(entries) = fs::read_dir(&preset_dir) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    // Only print the file stem if the extension is .toml
                    if p.extension().map_or(false, |ext| ext == "toml") {
                        let filename = p
                            .file_stem()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned();
                        println!("{}", filename);
                    }
                }
            }
        }
        Some(Commands::Set { preset_name }) => {
            let preset_path = format!("{}/{}.toml", &preset_dir, preset_name);
            if !PathBuf::from(&preset_path).exists() {
                eprintln!("Preset '{preset_name}' not found in {preset_dir}");
                std::process::exit(1);
            }
            println!("Activating preset '{preset_name}'...");
            set_config(&preset_path);
        }
        None => {
            println!("No subcommand given. Try --help or one of the subcommands:");
            println!("  toggle, list, set");
        }
    }
}

/// Spawn a new shell with STARSHIP_CONFIG pointing to `new_config`.
fn set_config(new_config: &str) {
    println!("Setting STARSHIP_CONFIG to {new_config}");

    Command::new("zsh")
        .env("STARSHIP_CONFIG", new_config)
        .arg("-c")
        .arg("exec zsh")
        .status()
        .expect("Failed to launch zsh");
}


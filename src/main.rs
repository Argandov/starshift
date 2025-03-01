use clap::{Parser, Subcommand};
use std::{
    env,
    fs,
    path::PathBuf,
    process::Command,
};

#[derive(Parser)]
#[command(author, version, about = "Starship preset manager in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List available Starship presets in ~/.config/starship_presets
    List,
    /// Set a specific preset by name (the .toml files must exist)
    Set {
        preset_name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // Where our starship config .toml files are stored
    let home = env::var("HOME").unwrap();
    let preset_dir = format!("{}/.config/starship_presets", home);

    match cli.command {
        Some(Commands::List) => {
            println!("Available presets in {preset_dir}:");
            if let Ok(entries) = fs::read_dir(&preset_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    // Print only if extension is .toml
                    if path.extension().map_or(false, |ext| ext == "toml") {
                        if let Some(stem) = path.file_stem() {
                            println!("{}", stem.to_string_lossy());
                        }
                    }
                }
            } else {
                eprintln!("Could not read directory {preset_dir}");
            }
        }
        Some(Commands::Set { preset_name }) => {
            let preset_path = format!("{}/{}.toml", &preset_dir, preset_name);
            if !PathBuf::from(&preset_path).exists() {
                eprintln!("Error: Preset '{preset_name}' not found in {preset_dir}");
                std::process::exit(1);
            }
            println!("Activating preset '{preset_name}'...");
            set_config(&preset_path);
        }
        None => {
            println!("No command given.\nUse `list` to see presets or `set <preset>` to select one.\nExample: starshift set mypreset");
        }
    }
}

/// Spawns a new Zsh process with STARSHIP_CONFIG pointing to the given `.toml`.
fn set_config(new_config: &str) {
    println!("Setting STARSHIP_CONFIG to {new_config}");
    Command::new("zsh")
        .env("STARSHIP_CONFIG", new_config)
        // Use `exec zsh` so that once inside, it replaces itself with zsh
        .arg("-c")
        .arg("exec zsh")
        .status()
        .expect("Failed to launch zsh");
}


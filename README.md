# Starshift

A simple CLI to manage and switch between multiple [Starship](https://starship.rs/) prompt configurations.  

## Overview

- **Goal:** Quickly switch between Starship prompt “presets” (e.g., a minimal prompt vs. a verbose one).
- **Language:** [Rust](https://www.rust-lang.org/) – coded with the aid of ChatGPT (More on that on #Disclaimer).
- **Motivation:**
  - I frequently had an overly verbose prompt (Git, Python venv, Cloud environment, etc.) when I didn’t need all that info 24/7.
  - This tool allows a default, minimal prompt, and an easy “switch” to a more detailed prompt only when I need it.

## Features

- **List** all `.toml` presets in `~/.config/starship_presets`.
- **Set** any preset as the active `STARSHIP_CONFIG` and spawn a new shell.

## Use cases / Narrative

- **Multiple Starship Configs:** Maybe you have separate `.toml` files for Git, Python venv, AWS, GCP, Azure, minimal, or super-verbose modes. Starshift lets you pick whichever one fits your current workflow.
- **Session-Specific Verbosity:** Start with a minimal prompt, then switch to a verbose config when you jump into a project needing Git status, Cloud profile, Python env details, etc. Once you exit that shell, you’re back to minimal.
- **Less Clutter, On Demand:** Avoid a cluttered prompt *until* you really need those extra indicators.

## Important

1. **Temporary Sessions**
   - When you run `starshift set <preset>`, it spawns **a new Zsh process**. That process inherits your existing environment (like Python venv or `AWS_PROFILE`) but **does not** inherit your in-memory command history. It’s effectively like running `exec zsh`.
   - Opening a brand-new terminal afterward defaults back to whatever you set in your `~/.zshrc`.

2. **Shell Assumption**
   - Starshift is currently tailored for Zsh. If you’re using Fish, Bash, or another shell, you’ll need to make minor tweaks in `main.rs` (e.g., replace `zsh` with `bash` or `fish`).

3. **History Handling**
   - In-memory history from your previous shell session isn’t carried over. However, if you use Zsh’s shared history options (`setopt INC_APPEND_HISTORY SHARE_HISTORY`), you’ll see near-seamless continuity of commands saved to disk.

## Requirements

- **ZSH support:** This program assumes you'll use Zsh as your main, default shell. Minor adjustments need to be done in the main.rs program to fit your needs for Fish, Bash, etc.
- **Rust, Cargo:** `cargo` installed.
- **Current starship configuration:** You need to have already installed starship ([starship.rs](https://starship.rs)), and Nerd Fonts (Optional).
- **Environment:** This works fine on Linux (Debian), also tested on MacOS.

## Installation

1. **Clone** the repo:
   ```bash
   git clone https://github.com/<your-username>/starshift.git
   cd starshift
   ```
2. Create a new directory: `mkdir $HOME/.config/starship_presets/`
3. Move/Copy your own starship TOML presets to this directory, and rename them according to your own specific wants (There are some random presets I placed in this project's [sample_configs](/sample_configs/) folder for some use cases - Don't treat them religiously, they're just samples I made on the fly to clarify the intention of this use cases when I was building the project). 
4. Compile or install with `cargo`. Place the binary wherever fits your environment best (i.e. `/usr/bin/`).
5. By default, you already have your default config file at $HOME/.config/starship.toml which be loaded every time a new shell opens. You can set this one to be your default minimal prompt, then move your custom ones to $HOME/.config/starship_presets/

## Usage

List Available Presets:

```bash
starshift list
```

This shows all .toml files in `~/.config/starship_presets/`.

Set a Preset:

```bash
starshift set minimal
```

*(It is not necessary to introduce the whole path, or file extension; just "minimal" instead of "minimal.toml")*

This will:

Spawn a new Zsh shell.

Set STARSHIP_CONFIG to ~/.config/starship_presets/minimal.toml.

Replace your current shell with the new one. Once you exit, you return to your prior shell (or environment).

## Disclaimer

This project was largely aided by ChatGPT, and I'm a Rust beginner doing this as a fun weekend experiment. To scratch my own itch for having different, switchable starship configurations and debloat my terminal prompt.

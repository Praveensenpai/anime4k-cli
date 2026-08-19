mod cli;
mod config;
mod doctor;
mod download;
mod extractor;
mod paths;
mod presets;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, Commands, InstallArgs, PresetArg, PresetArgs, UninstallArgs};
use colored::*;
use config::ConfigManager;
use paths::MpvPaths;
use presets::Preset;
use std::env;
use std::fs::File;

fn print_banner() {
    println!(
        "\n{} {}",
        "📺".bold(),
        "Anime4K GLSL Shaders & Hotkeys Manager for mpv"
            .bold()
            .cyan()
    );
    println!("{}\n", "─".repeat(55).dimmed());
}

fn print_keybinds_cheatsheet() {
    println!("\n{}", "🎮 Quick Hotkeys Reference (Press in mpv):".bold().yellow());
    println!("  {} {} : Mode A (HQ/Fast - Optimized for Anime)", "CTRL+1".bold().cyan(), "→".dimmed());
    println!("  {} {} : Mode B (HQ/Fast - Denoise + Deblur)", "CTRL+2".bold().cyan(), "→".dimmed());
    println!("  {} {} : Mode C (HQ/Fast - Deblur only)", "CTRL+3".bold().cyan(), "→".dimmed());
    println!("  {} {} : Mode A+A (Higher quality for 1080p -> 4K)", "CTRL+4".bold().cyan(), "→".dimmed());
    println!("  {} {} : Mode B+B", "CTRL+5".bold().cyan(), "→".dimmed());
    println!("  {} {} : Mode C+A", "CTRL+6".bold().cyan(), "→".dimmed());
    println!("  {} {} : Clear all GLSL shaders (Disable Anime4K)", "CTRL+0".bold().cyan(), "→".dimmed());
    println!("  {} {} : Show mpv statistics & active shaders", "Shift+I".bold().cyan(), "→".dimmed());
}

fn choose_preset_interactively() -> Result<Preset> {
    if let Ok(env_preset) = env::var("ANIME4K_PRESET") {
        if let Some(p) = Preset::from_str_loose(&env_preset) {
            return Ok(p);
        }
    }

    let items = vec![
        format!("1. {} ({})", Preset::High.name(), Preset::High.description()),
        format!("2. {} ({})", Preset::Low.name(), Preset::Low.description()),
    ];

    let selection = inquire::Select::new("🎮 Select Anime4K GPU preset:", items)
        .with_help_message("Use ↑/↓ arrow keys to navigate, Enter to select")
        .prompt();

    match selection {
        Ok(choice) => {
            if choice.starts_with("2.") {
                Ok(Preset::Low)
            } else {
                Ok(Preset::High)
            }
        }
        Err(_) => {
            // Fallback for non-interactive environments
            println!("  {} Using default preset: {}", "ℹ️".blue(), "Higher-end GPU (HQ)".bold());
            Ok(Preset::High)
        }
    }
}

fn execute_install(paths: &MpvPaths, args: InstallArgs) -> Result<()> {
    let preset: Preset = if let Some(arg) = args.preset {
        arg.into()
    } else if args.yes {
        Preset::High
    } else {
        choose_preset_interactively()?
    };

    println!("\n{} Selected preset: {}", "✨".green(), preset.name().bold().green());
    println!("{} MPV directory: {}\n", "📂".blue(), paths.root.display().to_string().bold());

    paths.ensure_dirs().context("Failed to create mpv directories")?;

    let temp_file = tempfile::Builder::new()
        .prefix("anime4k_")
        .suffix(".zip")
        .tempfile()
        .context("Failed to create temporary file for download")?;

    let temp_path = temp_file.path();
    download::download_file(preset.download_url(), temp_path, "Downloading Anime4K GLSL package...")?;

    println!("{} Extracting shaders to {}...", "📂".blue(), paths.shaders_dir().display().to_string().bold());
    let file = File::open(temp_path).context("Failed to open downloaded archive")?;
    let extracted = extractor::extract_shaders_and_input_conf(file, &paths.shaders_dir())?;
    println!("  {} Installed {} Anime4K shader files", "✔️".green(), extracted.shader_count.to_string().bold());

    let config_mgr = ConfigManager::new(paths);

    // Apply hotkeys to input.conf
    if !extracted.input_conf_content.is_empty() {
        config_mgr.configure_input_conf(&extracted.input_conf_content)?;
    }

    // Configure mpv.conf default shader line
    config_mgr.configure_mpv_conf(preset)?;

    println!(
        "\n{} Anime4K shaders ({}) successfully installed and configured for mpv!",
        "🎉".bold().green(),
        preset.name().bold()
    );

    print_keybinds_cheatsheet();
    Ok(())
}

fn execute_preset(paths: &MpvPaths, args: PresetArgs) -> Result<()> {
    let preset: Preset = args.preset.into();
    let config_mgr = ConfigManager::new(paths);

    if config_mgr.count_installed_shaders() == 0 {
        println!(
            "{} No Anime4K shaders found. Installing {} preset now...",
            "⚠️".yellow(),
            preset.name().bold()
        );
        return execute_install(
            paths,
            InstallArgs {
                preset: Some(args.preset),
                yes: true,
            },
        );
    }

    config_mgr.configure_mpv_conf(preset)?;
    println!(
        "{} Switched default Anime4K preset in mpv.conf to: {}",
        "✨".green().bold(),
        preset.name().bold().cyan()
    );
    Ok(())
}

fn execute_uninstall(paths: &MpvPaths, args: UninstallArgs) -> Result<()> {
    if !args.yes {
        let confirmed = inquire::Confirm::new("Are you sure you want to remove Anime4K from mpv?")
            .with_default(false)
            .prompt()
            .unwrap_or(false);

        if !confirmed {
            println!("{} Uninstall cancelled.", "ℹ️".blue());
            return Ok(());
        }
    }

    let config_mgr = ConfigManager::new(paths);
    let (mpv_cleaned, input_cleaned) = config_mgr.uninstall_configs()?;

    if mpv_cleaned {
        println!("  {} Removed Anime4K configuration from {}", "✔️".green(), "mpv.conf".bold());
    }
    if input_cleaned {
        println!("  {} Removed Anime4K keybindings from {}", "✔️".green(), "input.conf".bold());
    }

    if !args.keep_shaders {
        let count = config_mgr.uninstall_shaders()?;
        if count > 0 {
            println!("  {} Removed {} shader files from {}", "✔️".green(), count.to_string().bold(), paths.shaders_dir().display().to_string().bold());
        }
    } else {
        println!("  {} Kept shader files in {}", "ℹ️".blue(), paths.shaders_dir().display().to_string().bold());
    }

    println!("\n{} Anime4K uninstallation complete.", "🎉".green().bold());
    Ok(())
}

fn handle_interactive_menu(paths: &MpvPaths) -> Result<()> {
    let config_mgr = ConfigManager::new(paths);
    let current_preset = config_mgr.detect_current_preset();
    let shader_count = config_mgr.count_installed_shaders();

    let status_str = if shader_count > 0 {
        format!(
            "Installed ({}, {} shaders)",
            current_preset.map(|p| p.name()).unwrap_or("Custom"),
            shader_count
        )
    } else {
        "Not installed".to_string()
    };

    println!("Status: {}\n", status_str.cyan().bold());

    let options = vec![
        "1. Install / Update: Higher-end GPU (HQ - VL Shaders)",
        "2. Install / Update: Lower-end GPU (Fast - M/S Shaders)",
        "3. Switch Preset: Higher-end (HQ)",
        "4. Switch Preset: Lower-end (Fast)",
        "5. 🩺 Run Health Check (Doctor)",
        "6. 🎮 View Hotkeys Reference",
        "7. 🗑️  Uninstall Anime4K",
        "8. Exit",
    ];

    let choice = inquire::Select::new("Choose an action:", options).prompt();

    match choice {
        Ok(c) if c.starts_with("1.") => execute_install(
            paths,
            InstallArgs {
                preset: Some(PresetArg::High),
                yes: true,
            },
        ),
        Ok(c) if c.starts_with("2.") => execute_install(
            paths,
            InstallArgs {
                preset: Some(PresetArg::Low),
                yes: true,
            },
        ),
        Ok(c) if c.starts_with("3.") => execute_preset(
            paths,
            PresetArgs {
                preset: PresetArg::High,
            },
        ),
        Ok(c) if c.starts_with("4.") => execute_preset(
            paths,
            PresetArgs {
                preset: PresetArg::Low,
            },
        ),
        Ok(c) if c.starts_with("5.") => {
            doctor::run_doctor(paths);
            Ok(())
        }
        Ok(c) if c.starts_with("6.") => {
            print_keybinds_cheatsheet();
            Ok(())
        }
        Ok(c) if c.starts_with("7.") => execute_uninstall(
            paths,
            UninstallArgs {
                yes: false,
                keep_shaders: false,
            },
        ),
        _ => Ok(()),
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let paths = MpvPaths::resolve(cli.target_dir);

    print_banner();

    match cli.command {
        Some(Commands::Install(args)) => execute_install(&paths, args),
        Some(Commands::Preset(args)) => execute_preset(&paths, args),
        Some(Commands::Uninstall(args)) => execute_uninstall(&paths, args),
        Some(Commands::Doctor) => {
            doctor::run_doctor(&paths);
            Ok(())
        }
        Some(Commands::Info) => {
            print_keybinds_cheatsheet();
            Ok(())
        }
        None => handle_interactive_menu(&paths),
    }
}

mod cli;
mod config;
mod doctor;
mod download;
mod extractor;
mod paths;
mod presets;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, Commands, InstallArgs, ModeArg, ModeArgs, PresetArgs, TierArg, UninstallArgs};
use colored::*;
use config::ConfigManager;
use paths::MpvPaths;
use presets::{GpuTier, ShaderMode};
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
    println!("  {} {} : Mode A (HQ/Fast - Optimized for 1080p Anime)", "CTRL+1".bold().cyan(), "→".dimmed());
    println!("  {} {} : Mode B (HQ/Fast - Denoise + Deblur)", "CTRL+2".bold().cyan(), "→".dimmed());
    println!("  {} {} : Mode C (HQ/Fast - Deblur only)", "CTRL+3".bold().cyan(), "→".dimmed());
    println!("  {} {} : Mode A+A (Higher quality for 1080p -> 4K)", "CTRL+4".bold().cyan(), "→".dimmed());
    println!("  {} {} : Mode B+B (Higher quality denoise & deblur)", "CTRL+5".bold().cyan(), "→".dimmed());
    println!("  {} {} : Mode C+A (Higher quality deblur + upscale)", "CTRL+6".bold().cyan(), "→".dimmed());
    println!("  {} {} : Clear all GLSL shaders (Disable Anime4K)", "CTRL+0".bold().cyan(), "→".dimmed());
    println!("  {} {} : Show mpv statistics & active shaders", "Shift+I".bold().cyan(), "→".dimmed());
}

fn choose_tier_interactively() -> Result<GpuTier> {
    if let Ok(env_preset) = env::var("ANIME4K_PRESET") {
        if let Some(t) = GpuTier::from_str_loose(&env_preset) {
            return Ok(t);
        }
    }

    let items = vec![
        format!("1. {} ({})", GpuTier::High.name(), GpuTier::High.description()),
        format!("2. {} ({})", GpuTier::Low.name(), GpuTier::Low.description()),
    ];

    let selection = inquire::Select::new("🎮 Step 1/2: Select Anime4K GPU tier:", items)
        .with_help_message("Use ↑/↓ arrow keys to navigate, Enter to select")
        .prompt();

    match selection {
        Ok(choice) => {
            if choice.starts_with("2.") {
                Ok(GpuTier::Low)
            } else {
                Ok(GpuTier::High)
            }
        }
        Err(_) => {
            println!("  {} Using default tier: {}", "ℹ️".blue(), GpuTier::High.name().bold());
            Ok(GpuTier::High)
        }
    }
}

fn choose_mode_interactively() -> Result<ShaderMode> {
    if let Ok(env_mode) = env::var("ANIME4K_MODE") {
        if let Some(m) = ShaderMode::from_str_loose(&env_mode) {
            return Ok(m);
        }
    }

    let items: Vec<String> = ShaderMode::ALL
        .iter()
        .map(|mode| match mode {
            ShaderMode::ModeA => format!(
                "{} [{}] (⭐ Recommended: {})",
                mode.short_name().bold(),
                mode.hotkey().cyan(),
                mode.name()
            ),
            _ => format!(
                "{} [{}] ({})",
                mode.short_name().bold(),
                mode.hotkey().cyan(),
                mode.name()
            ),
        })
        .collect();

    let selection = inquire::Select::new("🎛️  Step 2/2: Select default startup shader mode:", items)
        .with_help_message("Use ↑/↓ arrow keys to navigate, Enter to select")
        .prompt();

    match selection {
        Ok(choice) => {
            for mode in ShaderMode::ALL {
                if choice.contains(mode.short_name()) {
                    return Ok(mode);
                }
            }
            Ok(ShaderMode::ModeA)
        }
        Err(_) => {
            println!("  {} Using default mode: {}", "ℹ️".blue(), "Mode A".bold());
            Ok(ShaderMode::ModeA)
        }
    }
}

fn execute_install(paths: &MpvPaths, args: InstallArgs) -> Result<()> {
    let tier: GpuTier = if let Some(arg) = args.preset {
        arg.into()
    } else if args.yes {
        GpuTier::High
    } else {
        choose_tier_interactively()?
    };

    let mode: ShaderMode = if let Some(arg) = args.mode {
        arg.into()
    } else if args.yes {
        ShaderMode::ModeA
    } else {
        choose_mode_interactively()?
    };

    println!("\n{} Selected GPU tier: {}", "✨".green(), tier.name().bold().green());
    println!("{} Selected startup mode: {}", "✨".green(), mode.short_name().bold().cyan());
    println!("{} MPV directory: {}\n", "📂".blue(), paths.root.display().to_string().bold());

    paths.ensure_dirs().context("Failed to create mpv directories")?;

    let temp_file = tempfile::Builder::new()
        .prefix("anime4k_")
        .suffix(".zip")
        .tempfile()
        .context("Failed to create temporary file for download")?;

    let temp_path = temp_file.path();
    download::download_file(tier.download_url(), temp_path, "Downloading Anime4K GLSL package...")?;

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
    config_mgr.configure_mpv_conf(tier, mode)?;

    println!(
        "\n{} Anime4K shaders ({}, {}) successfully installed and configured for mpv!",
        "🎉".bold().green(),
        tier.name().bold(),
        mode.short_name().bold().cyan()
    );

    print_keybinds_cheatsheet();
    Ok(())
}

fn execute_preset(paths: &MpvPaths, args: PresetArgs) -> Result<()> {
    let tier: GpuTier = args.tier.into();
    let mode: ShaderMode = args.mode.map(Into::into).unwrap_or(ShaderMode::ModeA);
    let config_mgr = ConfigManager::new(paths);

    if config_mgr.count_installed_shaders() == 0 {
        println!(
            "{} No Anime4K shaders found. Installing {} now...",
            "⚠️".yellow(),
            tier.name().bold()
        );
        return execute_install(
            paths,
            InstallArgs {
                preset: Some(args.tier),
                mode: args.mode,
                yes: true,
            },
        );
    }

    config_mgr.configure_mpv_conf(tier, mode)?;
    println!(
        "{} Switched default Anime4K tier & mode in mpv.conf to: {} ({})",
        "✨".green().bold(),
        tier.name().bold().cyan(),
        mode.short_name().bold().green()
    );
    Ok(())
}

fn execute_mode(paths: &MpvPaths, args: ModeArgs) -> Result<()> {
    let mode: ShaderMode = args.mode.into();
    let config_mgr = ConfigManager::new(paths);
    let (current_tier, _) = config_mgr.detect_current_setup();
    let tier = current_tier.unwrap_or(GpuTier::High);

    if config_mgr.count_installed_shaders() == 0 {
        println!(
            "{} No Anime4K shaders found. Installing shaders first...",
            "⚠️".yellow()
        );
        return execute_install(
            paths,
            InstallArgs {
                preset: Some(TierArg::High),
                mode: Some(args.mode),
                yes: true,
            },
        );
    }

    config_mgr.configure_mpv_conf(tier, mode)?;
    println!(
        "{} Switched default Anime4K startup mode in mpv.conf to: {}",
        "✨".green().bold(),
        mode.short_name().bold().cyan()
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
    let (current_tier, current_mode) = config_mgr.detect_current_setup();
    let shader_count = config_mgr.count_installed_shaders();

    let status_str = if shader_count > 0 {
        format!(
            "Installed ({}, Mode: {}, {} shaders)",
            current_tier.map(|t| t.name()).unwrap_or("Custom"),
            current_mode.map(|m| m.short_name()).unwrap_or("Custom"),
            shader_count
        )
    } else {
        "Not installed".to_string()
    };

    println!("Status: {}\n", status_str.cyan().bold());

    let options = vec![
        "1. 🚀 Install / Update Anime4K (Interactive GPU & Mode setup)",
        "2. 🎛️  Change Default Startup Mode (Mode A, B, C, A+A, B+B, C+A, or Off)",
        "3. 🎮 Switch GPU Tier (Higher-end HQ vs Lower-end Fast)",
        "4. 🩺 Run Health Check (Doctor)",
        "5. 📜 View Hotkeys Reference",
        "6. 🗑️  Uninstall Anime4K",
        "7. 🚪 Exit",
    ];

    let choice = inquire::Select::new("Choose an action:", options).prompt();

    match choice {
        Ok(c) if c.starts_with("1.") => execute_install(
            paths,
            InstallArgs {
                preset: None,
                mode: None,
                yes: false,
            },
        ),
        Ok(c) if c.starts_with("2.") => {
            let mode = choose_mode_interactively()?;
            let tier = current_tier.unwrap_or(GpuTier::High);
            config_mgr.configure_mpv_conf(tier, mode)?;
            println!(
                "{} Default startup mode updated to: {}",
                "✨".green().bold(),
                mode.short_name().cyan().bold()
            );
            Ok(())
        }
        Ok(c) if c.starts_with("3.") => {
            let tier = choose_tier_interactively()?;
            let mode = current_mode.unwrap_or(ShaderMode::ModeA);
            execute_preset(
                paths,
                PresetArgs {
                    tier: match tier {
                        GpuTier::High => TierArg::High,
                        GpuTier::Low => TierArg::Low,
                    },
                    mode: match mode {
                        ShaderMode::ModeA => Some(ModeArg::ModeA),
                        ShaderMode::ModeB => Some(ModeArg::ModeB),
                        ShaderMode::ModeC => Some(ModeArg::ModeC),
                        ShaderMode::ModeAA => Some(ModeArg::ModeAA),
                        ShaderMode::ModeBB => Some(ModeArg::ModeBB),
                        ShaderMode::ModeCA => Some(ModeArg::ModeCA),
                        ShaderMode::Disabled => Some(ModeArg::Disabled),
                    },
                },
            )
        }
        Ok(c) if c.starts_with("4.") => {
            doctor::run_doctor(paths);
            Ok(())
        }
        Ok(c) if c.starts_with("5.") => {
            print_keybinds_cheatsheet();
            Ok(())
        }
        Ok(c) if c.starts_with("6.") => execute_uninstall(
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
        Some(Commands::Mode(args)) => execute_mode(&paths, args),
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

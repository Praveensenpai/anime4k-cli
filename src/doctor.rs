use crate::config::ConfigManager;
use crate::paths::MpvPaths;
use colored::*;
use std::process::Command;

#[allow(dead_code)]
pub struct DoctorReport {
    pub mpv_installed: bool,
    pub mpv_version: Option<String>,
    pub config_dir_exists: bool,
    pub shader_count: usize,
    pub active_preset: Option<String>,
    pub input_conf_configured: bool,
    pub has_vo_gpu: bool,
    pub has_hwdec: bool,
}

pub fn run_doctor(paths: &MpvPaths) -> DoctorReport {
    println!("{}", "🩺 Running Anime4K & MPV System Health Check...".bold().cyan());
    println!("{}", "─".repeat(55).dimmed());

    // 1. Check MPV binary
    let (mpv_installed, mpv_version) = match Command::new("mpv").arg("--version").output() {
        Ok(output) if output.status.success() => {
            let ver_line = String::from_utf8_lossy(&output.stdout)
                .lines()
                .next()
                .unwrap_or("mpv (unknown version)")
                .to_string();
            (true, Some(ver_line))
        }
        _ => (false, None),
    };

    if mpv_installed {
        println!(
            "  {} MPV Binary: {}",
            "✔️".green(),
            mpv_version.as_deref().unwrap_or("Installed").green()
        );
    } else {
        println!(
            "  {} MPV Binary: {} (Make sure mpv is installed and in your PATH)",
            "⚠️".yellow(),
            "Not found in PATH".yellow()
        );
    }

    // 2. Check Paths
    let config_dir_exists = paths.root.exists();
    println!(
        "  {} MPV Config Dir: {} ({})",
        if config_dir_exists { "✔️".green() } else { "ℹ️".blue() },
        paths.root.display().to_string().bold(),
        if config_dir_exists { "Exists" } else { "Will be created" }
    );

    // 3. Check Shaders
    let config_mgr = ConfigManager::new(paths);
    let shader_count = config_mgr.count_installed_shaders();
    if shader_count > 0 {
        println!(
            "  {} Anime4K Shaders: {} found in {}",
            "✔️".green(),
            format!("{} shader files", shader_count).green().bold(),
            paths.shaders_dir().display().to_string().dimmed()
        );
    } else {
        println!(
            "  {} Anime4K Shaders: {} in {}",
            "❌".red(),
            "0 shaders found".red().bold(),
            paths.shaders_dir().display().to_string().dimmed()
        );
    }

    // 4. Check mpv.conf & Preset
    let active_preset = config_mgr.detect_current_preset();
    let mpv_conf_path = paths.mpv_conf();
    let mut has_vo_gpu = false;
    let mut has_hwdec = false;

    if mpv_conf_path.exists() {
        let content = std::fs::read_to_string(&mpv_conf_path).unwrap_or_default();
        has_vo_gpu = content.contains("vo=gpu") || content.contains("vo=gpu-next");
        has_hwdec = content.contains("hwdec=");

        if let Some(preset) = active_preset {
            println!(
                "  {} Default Preset: {} (in {})",
                "✔️".green(),
                preset.to_string().cyan().bold(),
                "mpv.conf".bold()
            );
        } else {
            println!(
                "  {} Default Preset: {} in {}",
                "⚠️".yellow(),
                "No active Anime4K preset".yellow(),
                "mpv.conf".bold()
            );
        }
    } else {
        println!(
            "  {} mpv.conf: {} (Run `anime4k install` to generate)",
            "ℹ️".blue(),
            "File does not exist yet".dimmed()
        );
    }

    // 5. Check input.conf
    let input_conf_path = paths.input_conf();
    let mut input_conf_configured = false;
    if input_conf_path.exists() {
        let content = std::fs::read_to_string(&input_conf_path).unwrap_or_default();
        if content.contains("Anime4K") {
            input_conf_configured = true;
            println!(
                "  {} Keybindings: {} mapped in {}",
                "✔️".green(),
                "Anime4K hotkeys (CTRL+0..6)".green().bold(),
                "input.conf".bold()
            );
        } else {
            println!(
                "  {} Keybindings: {} in {}",
                "⚠️".yellow(),
                "No Anime4K hotkeys".yellow(),
                "input.conf".bold()
            );
        }
    } else {
        println!(
            "  {} input.conf: {} (Run `anime4k install` to generate)",
            "ℹ️".blue(),
            "File does not exist yet".dimmed()
        );
    }

    println!("{}", "─".repeat(55).dimmed());

    // Recommendations
    if shader_count == 0 {
        println!(
            "{} Run {} to download shaders and configure mpv.",
            "💡 Tip:".yellow().bold(),
            "`anime4k install`".cyan().bold()
        );
    } else if active_preset.is_none() {
        println!(
            "{} Run {} to activate a GPU preset.",
            "💡 Tip:".yellow().bold(),
            "`anime4k preset high` or `anime4k preset low`".cyan().bold()
        );
    } else {
        println!(
            "{} Anime4K is fully installed and ready to use in mpv!",
            "🎉 All good:".green().bold()
        );
        println!("   Press {} inside mpv to toggle presets.", "CTRL+0..6 / CTRL+1 / CTRL+2".cyan());
    }

    DoctorReport {
        mpv_installed,
        mpv_version,
        config_dir_exists,
        shader_count,
        active_preset: active_preset.map(|p| p.to_string()),
        input_conf_configured,
        has_vo_gpu,
        has_hwdec,
    }
}

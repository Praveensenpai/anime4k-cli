use crate::presets::Preset;
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "anime4k",
    author = "Praveen <praveensenpai>",
    version = "1.0.0",
    about = "📺 Modern automated Anime4K GLSL shader and configuration manager for mpv",
    long_about = "Anime4K is a set of open-source, high-quality real-time anime upscaling and restoration shaders for mpv.\nThis CLI automates installation, preset switching, health checks, and hotkeys configuration."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Override the target mpv configuration directory
    #[arg(short, long, global = true)]
    pub target_dir: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Install Anime4K shaders, hotkeys, and configure mpv.conf
    Install(InstallArgs),

    /// Switch active Anime4K preset in mpv.conf instantly
    Preset(PresetArgs),

    /// Remove Anime4K shaders and clean up configuration
    Uninstall(UninstallArgs),

    /// Run diagnostic health check on mpv and Anime4K setup
    Doctor,

    /// Show Anime4K hotkeys and modes reference
    Info,
}

#[derive(Args, Debug)]
pub struct InstallArgs {
    /// GPU preset to install (high / low)
    #[arg(short, long, value_enum)]
    pub preset: Option<PresetArg>,

    /// Skip interactive prompts and accept defaults
    #[arg(short, long)]
    pub yes: bool,
}

#[derive(Args, Debug)]
pub struct PresetArgs {
    /// Preset to switch to (high / low)
    #[arg(value_enum)]
    pub preset: PresetArg,
}

#[derive(Args, Debug)]
pub struct UninstallArgs {
    /// Skip confirmation prompt
    #[arg(short, long)]
    pub yes: bool,

    /// Keep shader files in shaders/ directory, only clean configs
    #[arg(long)]
    pub keep_shaders: bool,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresetArg {
    #[value(name = "high", alias = "hq", alias = "higher")]
    High,
    #[value(name = "low", alias = "fast", alias = "lower")]
    Low,
}

impl From<PresetArg> for Preset {
    fn from(arg: PresetArg) -> Self {
        match arg {
            PresetArg::High => Preset::High,
            PresetArg::Low => Preset::Low,
        }
    }
}

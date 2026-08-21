use crate::presets::{GpuTier, ShaderMode};
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "anime4k",
    author = "Praveen <praveensenpai>",
    version = "1.0.2",
    about = "📺 Modern automated Anime4K GLSL shader and configuration manager for mpv",
    long_about = "Anime4K is a set of open-source, high-quality real-time anime upscaling and restoration shaders for mpv.\nThis CLI automates installation, preset & mode switching, health checks, and hotkeys configuration."
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
    /// Install Anime4K shaders, hotkeys, and configure default mode in mpv.conf
    Install(InstallArgs),

    /// Switch GPU preset tier (high-end HQ vs low-end Fast)
    Preset(PresetArgs),

    /// Change default startup shader mode in mpv.conf (A, B, C, A+A, B+B, C+A, or Off)
    Mode(ModeArgs),

    /// Remove Anime4K shaders and clean up configuration
    Uninstall(UninstallArgs),

    /// Run diagnostic health check on mpv and Anime4K setup
    Doctor,

    /// Show Anime4K hotkeys and modes reference
    Info,
}

#[derive(Args, Debug)]
pub struct InstallArgs {
    /// GPU tier to install (high / low)
    #[arg(short, long, alias = "tier", value_enum)]
    pub preset: Option<TierArg>,

    /// Default shader mode to enable at startup (a, b, c, a+a, b+b, c+a, or off)
    #[arg(short, long, value_enum)]
    pub mode: Option<ModeArg>,

    /// Skip interactive prompts and accept defaults
    #[arg(short, long)]
    pub yes: bool,
}

#[derive(Args, Debug)]
pub struct PresetArgs {
    /// GPU preset tier (high / low)
    #[arg(value_enum)]
    pub tier: TierArg,

    /// Optional shader mode to set (default: Mode A)
    #[arg(short, long, value_enum)]
    pub mode: Option<ModeArg>,
}

#[derive(Args, Debug)]
pub struct ModeArgs {
    /// Shader mode to activate at startup (a, b, c, a+a, b+b, c+a, or off)
    #[arg(value_enum)]
    pub mode: ModeArg,
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
pub enum TierArg {
    #[value(name = "high", alias = "hq", alias = "higher", alias = "higher-end")]
    High,
    #[value(name = "low", alias = "fast", alias = "lower", alias = "lower-end")]
    Low,
}

impl From<TierArg> for GpuTier {
    fn from(arg: TierArg) -> Self {
        match arg {
            TierArg::High => GpuTier::High,
            TierArg::Low => GpuTier::Low,
        }
    }
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModeArg {
    #[value(name = "a", alias = "mode-a", alias = "mode_a", alias = "1")]
    ModeA,
    #[value(name = "b", alias = "mode-b", alias = "mode_b", alias = "2")]
    ModeB,
    #[value(name = "c", alias = "mode-c", alias = "mode_c", alias = "3")]
    ModeC,
    #[value(name = "a+a", alias = "aa", alias = "mode-a+a", alias = "4")]
    ModeAA,
    #[value(name = "b+b", alias = "bb", alias = "mode-b+b", alias = "5")]
    ModeBB,
    #[value(name = "c+a", alias = "ca", alias = "mode-c+a", alias = "6")]
    ModeCA,
    #[value(name = "off", alias = "none", alias = "disabled", alias = "0", alias = "clear")]
    Disabled,
}

impl From<ModeArg> for ShaderMode {
    fn from(arg: ModeArg) -> Self {
        match arg {
            ModeArg::ModeA => ShaderMode::ModeA,
            ModeArg::ModeB => ShaderMode::ModeB,
            ModeArg::ModeC => ShaderMode::ModeC,
            ModeArg::ModeAA => ShaderMode::ModeAA,
            ModeArg::ModeBB => ShaderMode::ModeBB,
            ModeArg::ModeCA => ShaderMode::ModeCA,
            ModeArg::Disabled => ShaderMode::Disabled,
        }
    }
}

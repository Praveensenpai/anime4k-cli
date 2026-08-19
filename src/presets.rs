use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuTier {
    High,
    Low,
}

impl GpuTier {
    #[allow(dead_code)]
    pub const ALL: [GpuTier; 2] = [GpuTier::High, GpuTier::Low];

    pub fn name(&self) -> &'static str {
        match self {
            GpuTier::High => "Higher-end GPU (HQ)",
            GpuTier::Low => "Lower-end GPU (Fast)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            GpuTier::High => {
                "GTX 1080, RTX 2070/3060+, RX 5700XT/6600XT+ [HQ - VL Shaders]"
            }
            GpuTier::Low => {
                "GTX 980, GTX 1060, RX 570, Integrated GPUs [Fast - M/S Shaders]"
            }
        }
    }

    pub fn download_url(&self) -> &'static str {
        match self {
            GpuTier::High => {
                "https://github.com/Tama47/Anime4K/releases/download/v4.0.1/GLSL_Mac_Linux_High-end.zip"
            }
            GpuTier::Low => {
                "https://github.com/Tama47/Anime4K/releases/download/v4.0.1/GLSL_Mac_Linux_Low-end.zip"
            }
        }
    }

    pub fn from_str_loose(s: &str) -> Option<GpuTier> {
        match s.trim().to_lowercase().as_str() {
            "1" | "high" | "hq" | "high-end" | "higher" | "higher-end" => Some(GpuTier::High),
            "2" | "low" | "fast" | "low-end" | "lower" | "lower-end" => Some(GpuTier::Low),
            _ => None,
        }
    }
}

impl fmt::Display for GpuTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderMode {
    ModeA,
    ModeB,
    ModeC,
    ModeAA,
    ModeBB,
    ModeCA,
    Disabled,
}

impl ShaderMode {
    pub const ALL: [ShaderMode; 7] = [
        ShaderMode::ModeA,
        ShaderMode::ModeB,
        ShaderMode::ModeC,
        ShaderMode::ModeAA,
        ShaderMode::ModeBB,
        ShaderMode::ModeCA,
        ShaderMode::Disabled,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            ShaderMode::ModeA => "Mode A (Restore + Upscale - Optimized for 1080p anime)",
            ShaderMode::ModeB => "Mode B (Denoise + Deblur - Great for older/grainy anime)",
            ShaderMode::ModeC => "Mode C (Deblur only - For clean anime)",
            ShaderMode::ModeAA => "Mode A+A (High-res upscale for 1080p -> 4K)",
            ShaderMode::ModeBB => "Mode B+B (High-res denoise & deblur)",
            ShaderMode::ModeCA => "Mode C+A (High-res deblur + upscale)",
            ShaderMode::Disabled => "Disabled by default (Shaders OFF at startup, toggle with hotkeys)",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            ShaderMode::ModeA => "Mode A",
            ShaderMode::ModeB => "Mode B",
            ShaderMode::ModeC => "Mode C",
            ShaderMode::ModeAA => "Mode A+A",
            ShaderMode::ModeBB => "Mode B+B",
            ShaderMode::ModeCA => "Mode C+A",
            ShaderMode::Disabled => "Disabled",
        }
    }

    pub fn hotkey(&self) -> &'static str {
        match self {
            ShaderMode::ModeA => "CTRL+1",
            ShaderMode::ModeB => "CTRL+2",
            ShaderMode::ModeC => "CTRL+3",
            ShaderMode::ModeAA => "CTRL+4",
            ShaderMode::ModeBB => "CTRL+5",
            ShaderMode::ModeCA => "CTRL+6",
            ShaderMode::Disabled => "CTRL+0",
        }
    }

    pub fn glsl_shaders_line(&self, tier: GpuTier) -> Option<String> {
        let line = match (tier, self) {
            (GpuTier::High, ShaderMode::ModeA) => {
                "glsl-shaders=\"~~/shaders/Anime4K_Clamp_Highlights.glsl:~~/shaders/Anime4K_Restore_CNN_VL.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_VL.glsl:~~/shaders/Anime4K_AutoDownscalePre_x2.glsl:~~/shaders/Anime4K_AutoDownscalePre_x4.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_M.glsl\""
            }
            (GpuTier::High, ShaderMode::ModeB) => {
                "glsl-shaders=\"~~/shaders/Anime4K_Clamp_Highlights.glsl:~~/shaders/Anime4K_Restore_CNN_Soft_VL.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_VL.glsl:~~/shaders/Anime4K_AutoDownscalePre_x2.glsl:~~/shaders/Anime4K_AutoDownscalePre_x4.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_M.glsl\""
            }
            (GpuTier::High, ShaderMode::ModeC) => {
                "glsl-shaders=\"~~/shaders/Anime4K_Clamp_Highlights.glsl:~~/shaders/Anime4K_Upscale_Denoise_CNN_x2_VL.glsl:~~/shaders/Anime4K_AutoDownscalePre_x2.glsl:~~/shaders/Anime4K_AutoDownscalePre_x4.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_M.glsl\""
            }
            (GpuTier::High, ShaderMode::ModeAA) => {
                "glsl-shaders=\"~~/shaders/Anime4K_Clamp_Highlights.glsl:~~/shaders/Anime4K_Restore_CNN_VL.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_VL.glsl:~~/shaders/Anime4K_Restore_CNN_M.glsl:~~/shaders/Anime4K_AutoDownscalePre_x2.glsl:~~/shaders/Anime4K_AutoDownscalePre_x4.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_M.glsl\""
            }
            (GpuTier::High, ShaderMode::ModeBB) => {
                "glsl-shaders=\"~~/shaders/Anime4K_Clamp_Highlights.glsl:~~/shaders/Anime4K_Restore_CNN_Soft_VL.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_VL.glsl:~~/shaders/Anime4K_Restore_CNN_Soft_M.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_M.glsl\""
            }
            (GpuTier::High, ShaderMode::ModeCA) => {
                "glsl-shaders=\"~~/shaders/Anime4K_Clamp_Highlights.glsl:~~/shaders/Anime4K_Upscale_Denoise_CNN_x2_VL.glsl:~~/shaders/Anime4K_AutoDownscalePre_x2.glsl:~~/shaders/Anime4K_AutoDownscalePre_x4.glsl:~~/shaders/Anime4K_Restore_CNN_M.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_M.glsl\""
            }

            (GpuTier::Low, ShaderMode::ModeA) => {
                "glsl-shaders=\"~~/shaders/Anime4K_Clamp_Highlights.glsl:~~/shaders/Anime4K_Restore_CNN_M.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_M.glsl:~~/shaders/Anime4K_AutoDownscalePre_x2.glsl:~~/shaders/Anime4K_AutoDownscalePre_x4.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_S.glsl\""
            }
            (GpuTier::Low, ShaderMode::ModeB) => {
                "glsl-shaders=\"~~/shaders/Anime4K_Clamp_Highlights.glsl:~~/shaders/Anime4K_Restore_CNN_Soft_M.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_M.glsl:~~/shaders/Anime4K_AutoDownscalePre_x2.glsl:~~/shaders/Anime4K_AutoDownscalePre_x4.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_S.glsl\""
            }
            (GpuTier::Low, ShaderMode::ModeC) => {
                "glsl-shaders=\"~~/shaders/Anime4K_Clamp_Highlights.glsl:~~/shaders/Anime4K_Upscale_Denoise_CNN_x2_M.glsl:~~/shaders/Anime4K_AutoDownscalePre_x2.glsl:~~/shaders/Anime4K_AutoDownscalePre_x4.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_S.glsl\""
            }
            (GpuTier::Low, ShaderMode::ModeAA) => {
                "glsl-shaders=\"~~/shaders/Anime4K_Clamp_Highlights.glsl:~~/shaders/Anime4K_Restore_CNN_M.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_M.glsl:~~/shaders/Anime4K_Restore_CNN_S.glsl:~~/shaders/Anime4K_AutoDownscalePre_x2.glsl:~~/shaders/Anime4K_AutoDownscalePre_x4.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_S.glsl\""
            }
            (GpuTier::Low, ShaderMode::ModeBB) => {
                "glsl-shaders=\"~~/shaders/Anime4K_Clamp_Highlights.glsl:~~/shaders/Anime4K_Restore_CNN_Soft_M.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_M.glsl:~~/shaders/Anime4K_Restore_CNN_Soft_S.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_S.glsl\""
            }
            (GpuTier::Low, ShaderMode::ModeCA) => {
                "glsl-shaders=\"~~/shaders/Anime4K_Clamp_Highlights.glsl:~~/shaders/Anime4K_Upscale_Denoise_CNN_x2_M.glsl:~~/shaders/Anime4K_AutoDownscalePre_x2.glsl:~~/shaders/Anime4K_AutoDownscalePre_x4.glsl:~~/shaders/Anime4K_Restore_CNN_S.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_S.glsl\""
            }

            (_, ShaderMode::Disabled) => return None,
        };

        Some(line.to_string())
    }

    pub fn header_comment(&self, tier: GpuTier) -> String {
        let tag = match tier {
            GpuTier::High => "higher-end GPU (HQ)",
            GpuTier::Low => "lower-end GPU (Fast)",
        };

        match self {
            ShaderMode::Disabled => {
                format!("# Anime4K shaders configured for {} (default: disabled on startup, use hotkeys)", tag)
            }
            mode => format!("# Optimized shaders for {}: {}", tag, mode.short_name()),
        }
    }

    pub fn from_str_loose(s: &str) -> Option<ShaderMode> {
        match s.trim().to_lowercase().replace(['-', '_', ' '], "").as_str() {
            "1" | "a" | "modea" => Some(ShaderMode::ModeA),
            "2" | "b" | "modeb" => Some(ShaderMode::ModeB),
            "3" | "c" | "modec" => Some(ShaderMode::ModeC),
            "4" | "aa" | "a+a" | "modeaa" | "modea+a" => Some(ShaderMode::ModeAA),
            "5" | "bb" | "b+b" | "modebb" | "modeb+b" => Some(ShaderMode::ModeBB),
            "6" | "ca" | "c+a" | "modeca" | "modec+a" => Some(ShaderMode::ModeCA),
            "0" | "off" | "none" | "disabled" | "clear" => Some(ShaderMode::Disabled),
            _ => None,
        }
    }
}

impl fmt::Display for ShaderMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.short_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_from_str() {
        assert_eq!(GpuTier::from_str_loose("high"), Some(GpuTier::High));
        assert_eq!(GpuTier::from_str_loose("1"), Some(GpuTier::High));
        assert_eq!(GpuTier::from_str_loose("low"), Some(GpuTier::Low));
        assert_eq!(GpuTier::from_str_loose("2"), Some(GpuTier::Low));
        assert_eq!(GpuTier::from_str_loose("invalid"), None);
    }

    #[test]
    fn test_mode_from_str() {
        assert_eq!(ShaderMode::from_str_loose("a"), Some(ShaderMode::ModeA));
        assert_eq!(ShaderMode::from_str_loose("b"), Some(ShaderMode::ModeB));
        assert_eq!(ShaderMode::from_str_loose("c"), Some(ShaderMode::ModeC));
        assert_eq!(ShaderMode::from_str_loose("a+a"), Some(ShaderMode::ModeAA));
        assert_eq!(ShaderMode::from_str_loose("aa"), Some(ShaderMode::ModeAA));
        assert_eq!(ShaderMode::from_str_loose("b+b"), Some(ShaderMode::ModeBB));
        assert_eq!(ShaderMode::from_str_loose("c+a"), Some(ShaderMode::ModeCA));
        assert_eq!(ShaderMode::from_str_loose("none"), Some(ShaderMode::Disabled));
        assert_eq!(ShaderMode::from_str_loose("off"), Some(ShaderMode::Disabled));
        assert_eq!(ShaderMode::from_str_loose("0"), Some(ShaderMode::Disabled));
    }

    #[test]
    fn test_all_glsl_lines() {
        for tier in GpuTier::ALL {
            for mode in ShaderMode::ALL {
                if mode == ShaderMode::Disabled {
                    assert!(mode.glsl_shaders_line(tier).is_none());
                } else {
                    let line = mode.glsl_shaders_line(tier).unwrap();
                    assert!(line.starts_with("glsl-shaders=\"~~/shaders/Anime4K_"));
                }
                assert!(!mode.header_comment(tier).is_empty());
            }
        }
    }
}

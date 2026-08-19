use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    High,
    Low,
}

impl Preset {
    #[allow(dead_code)]
    pub const ALL: [Preset; 2] = [Preset::High, Preset::Low];

    pub fn name(&self) -> &'static str {
        match self {
            Preset::High => "Higher-end GPU (HQ)",
            Preset::Low => "Lower-end GPU (Fast)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Preset::High => {
                "GTX 1080, RTX 2070/3060+, RX 5700XT/6600XT+ [HQ - VL Shaders (Restore_CNN_VL, Upscale_CNN_x2_VL)]"
            }
            Preset::Low => {
                "GTX 980, GTX 1060, RX 570, Integrated GPUs [Fast - M/S Shaders (Restore_CNN_M, Upscale_CNN_x2_M)]"
            }
        }
    }

    pub fn download_url(&self) -> &'static str {
        match self {
            Preset::High => {
                "https://github.com/Tama47/Anime4K/releases/download/v4.0.1/GLSL_Mac_Linux_High-end.zip"
            }
            Preset::Low => {
                "https://github.com/Tama47/Anime4K/releases/download/v4.0.1/GLSL_Mac_Linux_Low-end.zip"
            }
        }
    }

    pub fn glsl_shaders_line(&self) -> &'static str {
        match self {
            Preset::High => {
                "glsl-shaders=\"~~/shaders/Anime4K_Clamp_Highlights.glsl:~~/shaders/Anime4K_Restore_CNN_VL.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_VL.glsl:~~/shaders/Anime4K_AutoDownscalePre_x2.glsl:~~/shaders/Anime4K_AutoDownscalePre_x4.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_M.glsl\""
            }
            Preset::Low => {
                "glsl-shaders=\"~~/shaders/Anime4K_Clamp_Highlights.glsl:~~/shaders/Anime4K_Restore_CNN_M.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_M.glsl:~~/shaders/Anime4K_AutoDownscalePre_x2.glsl:~~/shaders/Anime4K_AutoDownscalePre_x4.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_S.glsl\""
            }
        }
    }

    pub fn header_comment(&self) -> &'static str {
        match self {
            Preset::High => "# Optimized shaders for higher-end GPU: Mode A (HQ)",
            Preset::Low => "# Optimized shaders for lower-end GPU: Mode A (Fast)",
        }
    }

    pub fn from_str_loose(s: &str) -> Option<Preset> {
        match s.trim().to_lowercase().as_str() {
            "1" | "high" | "hq" | "high-end" | "higher" | "higher-end" => Some(Preset::High),
            "2" | "low" | "fast" | "low-end" | "lower" | "lower-end" => Some(Preset::Low),
            _ => None,
        }
    }
}

impl fmt::Display for Preset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preset_from_str() {
        assert_eq!(Preset::from_str_loose("high"), Some(Preset::High));
        assert_eq!(Preset::from_str_loose("1"), Some(Preset::High));
        assert_eq!(Preset::from_str_loose("hq"), Some(Preset::High));
        assert_eq!(Preset::from_str_loose("low"), Some(Preset::Low));
        assert_eq!(Preset::from_str_loose("2"), Some(Preset::Low));
        assert_eq!(Preset::from_str_loose("fast"), Some(Preset::Low));
        assert_eq!(Preset::from_str_loose("invalid"), None);
    }

    #[test]
    fn test_preset_properties() {
        for preset in Preset::ALL {
            assert!(!preset.name().is_empty());
            assert!(!preset.description().is_empty());
            assert!(preset.download_url().starts_with("https://"));
            assert!(preset.glsl_shaders_line().contains("glsl-shaders="));
            assert!(preset.header_comment().starts_with("#"));
        }
    }
}

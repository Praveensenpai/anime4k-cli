use crate::paths::MpvPaths;
use crate::presets::{GpuTier, ShaderMode};
use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub const BLOCK_START: &str = "# >>> anime4k-managed-start >>>";
pub const BLOCK_END: &str = "# <<< anime4k-managed-end <<<";

pub struct ConfigManager<'a> {
    pub paths: &'a MpvPaths,
}

impl<'a> ConfigManager<'a> {
    pub fn new(paths: &'a MpvPaths) -> Self {
        Self { paths }
    }

    /// Backup a file if it exists into the backup directory
    pub fn backup_file(&self, file_path: &Path) -> Result<Option<PathBuf>> {
        if !file_path.exists() {
            return Ok(None);
        }

        let backup_dir = self.paths.backup_dir();
        fs::create_dir_all(&backup_dir)?;

        let filename = file_path
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("file");

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let backup_name = format!("{}.{}.bak", filename, timestamp);
        let backup_path = backup_dir.join(backup_name);

        fs::copy(file_path, &backup_path).with_context(|| {
            format!("Failed to backup {:?} to {:?}", file_path, backup_path)
        })?;

        Ok(Some(backup_path))
    }

    /// Apply preset configuration to mpv.conf
    pub fn configure_mpv_conf(&self, tier: GpuTier, mode: ShaderMode) -> Result<()> {
        let conf_path = self.paths.mpv_conf();
        let backup = self.backup_file(&conf_path)?;
        if let Some(b) = backup {
            println!("  {} Backed up existing {} to {:?}", "💾".blue(), "mpv.conf".bold(), b);
        }

        let original_content = if conf_path.exists() {
            fs::read_to_string(&conf_path).unwrap_or_default()
        } else {
            String::new()
        };

        let header = mode.header_comment(tier);
        let managed_content = match mode.glsl_shaders_line(tier) {
            Some(glsl_line) => {
                format!("{}\n{}\n{}\n{}", BLOCK_START, header, glsl_line, BLOCK_END)
            }
            None => {
                format!(
                    "{}\n{}\n# Shaders are disabled by default at startup. Use hotkeys (CTRL+1..6) to activate.\n{}",
                    BLOCK_START, header, BLOCK_END
                )
            }
        };

        let new_content = Self::replace_or_append_block(&original_content, &managed_content, true);

        let mut file = fs::File::create(&conf_path)
            .with_context(|| format!("Failed to write to {:?}", conf_path))?;
        file.write_all(new_content.as_bytes())?;

        println!(
            "  {} Configured default preset in {} ({}, {})",
            "⚙️".green(),
            "mpv.conf".bold(),
            tier.name().bold(),
            mode.short_name().cyan().bold()
        );
        Ok(())
    }

    /// Apply Anime4K hotkeys to input.conf
    pub fn configure_input_conf(&self, bundled_input_conf: &str) -> Result<()> {
        let conf_path = self.paths.input_conf();
        let backup = self.backup_file(&conf_path)?;
        if let Some(b) = backup {
            println!("  {} Backed up existing {} to {:?}", "💾".blue(), "input.conf".bold(), b);
        }

        let original_content = if conf_path.exists() {
            fs::read_to_string(&conf_path).unwrap_or_default()
        } else {
            String::new()
        };

        let keybinds_clean = bundled_input_conf.trim();
        let managed_content = format!(
            "{}\n# Anime4K GLSL Hotkeys\n{}\n{}",
            BLOCK_START,
            keybinds_clean,
            BLOCK_END
        );

        let new_content = Self::replace_or_append_block(&original_content, &managed_content, false);

        let mut file = fs::File::create(&conf_path)
            .with_context(|| format!("Failed to write to {:?}", conf_path))?;
        file.write_all(new_content.as_bytes())?;

        println!("  {} Configured hotkeys in {}", "📝".green(), "input.conf".bold());
        Ok(())
    }

    /// Remove Anime4K managed blocks from mpv.conf and input.conf
    pub fn uninstall_configs(&self) -> Result<(bool, bool)> {
        let mut mpv_changed = false;
        let mut input_changed = false;

        let mpv_path = self.paths.mpv_conf();
        if mpv_path.exists() {
            let content = fs::read_to_string(&mpv_path)?;
            let cleaned = Self::strip_managed_block(&content, true);
            if cleaned != content {
                self.backup_file(&mpv_path)?;
                fs::write(&mpv_path, cleaned)?;
                mpv_changed = true;
            }
        }

        let input_path = self.paths.input_conf();
        if input_path.exists() {
            let content = fs::read_to_string(&input_path)?;
            let cleaned = Self::strip_managed_block(&content, false);
            if cleaned != content {
                self.backup_file(&input_path)?;
                fs::write(&input_path, cleaned)?;
                input_changed = true;
            }
        }

        Ok((mpv_changed, input_changed))
    }

    /// Remove all Anime4K shaders from shaders/
    pub fn uninstall_shaders(&self) -> Result<usize> {
        let shaders_dir = self.paths.shaders_dir();
        if !shaders_dir.exists() {
            return Ok(0);
        }

        let mut removed = 0;
        for entry in fs::read_dir(&shaders_dir)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("Anime4K_") && name.ends_with(".glsl") {
                    fs::remove_file(&path)?;
                    removed += 1;
                }
            }
        }

        Ok(removed)
    }

    /// Replace existing managed block or legacy lines, or append to end
    fn replace_or_append_block(content: &str, managed_block: &str, is_mpv_conf: bool) -> String {
        // If managed block already exists, replace it
        if let Some(start_idx) = content.find(BLOCK_START) {
            if let Some(end_rel) = content[start_idx..].find(BLOCK_END) {
                let end_idx = start_idx + end_rel + BLOCK_END.len();
                let mut result = String::new();
                result.push_str(content[..start_idx].trim_end());
                if !result.is_empty() {
                    result.push_str("\n\n");
                }
                result.push_str(managed_block);
                let trailing = content[end_idx..].trim_start();
                if !trailing.is_empty() {
                    result.push_str("\n\n");
                    result.push_str(trailing);
                }
                result.push('\n');
                return result;
            }
        }

        // If legacy lines exist (e.g. from previous installer versions), strip them first
        let stripped = Self::strip_legacy_lines(content, is_mpv_conf);

        let mut result = stripped.trim_end().to_string();
        if !result.is_empty() {
            result.push_str("\n\n");
        }
        result.push_str(managed_block);
        result.push('\n');
        result
    }

    /// Strip managed block completely
    fn strip_managed_block(content: &str, is_mpv_conf: bool) -> String {
        let mut text = content.to_string();
        if let Some(start_idx) = text.find(BLOCK_START) {
            if let Some(end_rel) = text[start_idx..].find(BLOCK_END) {
                let end_idx = start_idx + end_rel + BLOCK_END.len();
                let mut stripped = String::new();
                stripped.push_str(text[..start_idx].trim_end());
                let trailing = text[end_idx..].trim_start();
                if !trailing.is_empty() {
                    if !stripped.is_empty() {
                        stripped.push_str("\n\n");
                    }
                    stripped.push_str(trailing);
                }
                if !stripped.is_empty() {
                    stripped.push('\n');
                }
                text = stripped;
            }
        }

        let final_stripped = Self::strip_legacy_lines(&text, is_mpv_conf);
        let trimmed = final_stripped.trim();
        if trimmed.is_empty() {
            String::new()
        } else {
            format!("{}\n", trimmed)
        }
    }

    /// Strip old unmanaged Anime4K lines (from previous shell script versions)
    fn strip_legacy_lines(content: &str, is_mpv_conf: bool) -> String {
        let mut lines: Vec<&str> = Vec::new();
        for line in content.lines() {
            let trim = line.trim();
            if is_mpv_conf {
                if trim.starts_with("# Optimized shaders for") && trim.contains("GPU:") {
                    continue;
                }
                if (trim.starts_with("glsl-shaders=") || trim.starts_with("#glsl-shaders="))
                    && trim.contains("Anime4K_")
                {
                    continue;
                }
            } else {
                if trim.contains("Anime4K") {
                    continue;
                }
            }
            lines.push(line);
        }
        lines.join("\n")
    }

    /// Detect the currently configured GPU tier and Shader mode from mpv.conf and input.conf
    pub fn detect_current_setup(&self) -> (Option<GpuTier>, Option<ShaderMode>) {
        let mpv_path = self.paths.mpv_conf();
        let input_path = self.paths.input_conf();

        let mpv_content = if mpv_path.exists() {
            fs::read_to_string(&mpv_path).unwrap_or_default()
        } else {
            String::new()
        };

        let input_content = if input_path.exists() {
            fs::read_to_string(&input_path).unwrap_or_default()
        } else {
            String::new()
        };

        let combined = format!("{}\n{}", mpv_content, input_content);

        let tier = if combined.contains("VL.glsl") || combined.contains("higher-end") || combined.contains("(HQ)") {
            Some(GpuTier::High)
        } else if combined.contains("Restore_CNN_M.glsl") || combined.contains("lower-end") || combined.contains("(Fast)") {
            Some(GpuTier::Low)
        } else {
            None
        };

        let mode = if mpv_content.contains("disabled on startup") {
            Some(ShaderMode::Disabled)
        } else if mpv_content.contains("Mode A+A") || (mpv_content.contains("Restore_CNN_") && mpv_content.matches("Restore_CNN_").count() >= 2) {
            Some(ShaderMode::ModeAA)
        } else if mpv_content.contains("Mode B+B") || (mpv_content.contains("Restore_CNN_Soft_") && mpv_content.matches("Restore_CNN_Soft_").count() >= 2) {
            Some(ShaderMode::ModeBB)
        } else if mpv_content.contains("Mode C+A") || (mpv_content.contains("Upscale_Denoise_") && mpv_content.contains("Restore_CNN_")) {
            Some(ShaderMode::ModeCA)
        } else if mpv_content.contains("Mode B") || mpv_content.contains("Restore_CNN_Soft_") {
            Some(ShaderMode::ModeB)
        } else if mpv_content.contains("Mode C") || mpv_content.contains("Upscale_Denoise_") {
            Some(ShaderMode::ModeC)
        } else if mpv_content.contains("Mode A") || mpv_content.contains("glsl-shaders=") && mpv_content.contains("Anime4K_") {
            Some(ShaderMode::ModeA)
        } else {
            None
        };

        (tier, mode)
    }

    /// Count installed Anime4K shader files
    pub fn count_installed_shaders(&self) -> usize {
        let shaders_dir = self.paths.shaders_dir();
        if !shaders_dir.exists() {
            return 0;
        }

        fs::read_dir(&shaders_dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        e.file_name()
                            .to_str()
                            .map(|n| n.starts_with("Anime4K_") && n.ends_with(".glsl"))
                            .unwrap_or(false)
                    })
                    .count()
            })
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_or_append_new_file() {
        let managed = format!("{}\nglsl-shaders=test\n{}", BLOCK_START, BLOCK_END);
        let result = ConfigManager::replace_or_append_block("", &managed, true);
        assert_eq!(result, format!("{}\n", managed));
    }

    #[test]
    fn test_replace_existing_managed_block() {
        let initial = format!(
            "vo=gpu\n{}\nglsl-shaders=old\n{}\nhwdec=auto\n",
            BLOCK_START, BLOCK_END
        );
        let new_managed = format!("{}\nglsl-shaders=new\n{}", BLOCK_START, BLOCK_END);
        let result = ConfigManager::replace_or_append_block(&initial, &new_managed, true);
        assert!(result.contains("vo=gpu"));
        assert!(result.contains("glsl-shaders=new"));
        assert!(!result.contains("glsl-shaders=old"));
        assert!(result.contains("hwdec=auto"));
    }

    #[test]
    fn test_strip_managed_block() {
        let content = format!(
            "vo=gpu\n\n{}\nglsl-shaders=test\n{}\n\nhwdec=auto\n",
            BLOCK_START, BLOCK_END
        );
        let result = ConfigManager::strip_managed_block(&content, true);
        assert_eq!(result, "vo=gpu\n\nhwdec=auto\n");
    }
}

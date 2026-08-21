use std::env;
use std::path::PathBuf;

pub struct MpvPaths {
    pub root: PathBuf,
}

impl MpvPaths {
    pub fn resolve(custom_dir: Option<PathBuf>) -> Self {
        let root = if let Some(dir) = custom_dir {
            dir
        } else if let Ok(mpv_home) = env::var("MPV_HOME") {
            PathBuf::from(mpv_home)
        } else {
            Self::default_mpv_dir()
        };

        Self { root }
    }

    fn default_mpv_dir() -> PathBuf {
        if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("mpv")
        } else if let Some(home) = dirs::home_dir() {
            home.join(".config").join("mpv")
        } else {
            PathBuf::from(".config/mpv")
        }
    }

    pub fn shaders_dir(&self) -> PathBuf {
        self.root.join("shaders")
    }

    pub fn mpv_conf(&self) -> PathBuf {
        self.root.join("mpv.conf")
    }

    pub fn input_conf(&self) -> PathBuf {
        self.root.join("input.conf")
    }

    pub fn backup_dir(&self) -> PathBuf {
        self.root.join("anime4k_backups")
    }

    pub fn ensure_dirs(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.root)?;
        std::fs::create_dir_all(self.shaders_dir())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_paths_resolution() {
        let custom = PathBuf::from("/tmp/custom_mpv");
        let paths = MpvPaths::resolve(Some(custom.clone()));
        assert_eq!(paths.root, custom);
        assert_eq!(paths.shaders_dir(), custom.join("shaders"));
        assert_eq!(paths.mpv_conf(), custom.join("mpv.conf"));
        assert_eq!(paths.input_conf(), custom.join("input.conf"));
        assert_eq!(paths.backup_dir(), custom.join("anime4k_backups"));
    }
}

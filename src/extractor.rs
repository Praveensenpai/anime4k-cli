use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::{Read, Seek};
use std::path::Path;
use zip::ZipArchive;

pub struct ExtractedPackage {
    pub shader_count: usize,
    pub input_conf_content: String,
}

pub fn extract_shaders_and_input_conf<R: Read + Seek>(
    reader: R,
    target_shaders_dir: &Path,
) -> Result<ExtractedPackage> {
    let mut archive = ZipArchive::new(reader).context("Failed to read zip archive")?;
    fs::create_dir_all(target_shaders_dir).with_context(|| {
        format!(
            "Failed to create target shaders directory at {:?}",
            target_shaders_dir
        )
    })?;

    let mut shader_count = 0;
    let mut input_conf_content = String::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let name = file.name().to_string();

        // Skip macOS metadata and directories
        if name.starts_with("__MACOSX") || name.contains("/._") || name.starts_with("._") {
            continue;
        }

        // Extract glsl shaders
        if name.ends_with(".glsl") {
            let file_name = Path::new(&name)
                .file_name()
                .context("Invalid shader filename in zip")?;
            let dest_path = target_shaders_dir.join(file_name);

            let mut dest_file = File::create(&dest_path)
                .with_context(|| format!("Failed to create shader file {:?}", dest_path))?;
            std::io::copy(&mut file, &mut dest_file)?;
            shader_count += 1;
        } else if name == "input.conf" || name.ends_with("/input.conf") {
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            input_conf_content = String::from_utf8_lossy(&bytes).to_string();
        }
    }

    Ok(ExtractedPackage {
        shader_count,
        input_conf_content,
    })
}

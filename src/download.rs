use anyhow::{Context, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;

pub fn download_file(url: &str, output_path: &Path, label: &str) -> Result<()> {
    println!("{} {}", "📦".bold(), label.cyan().bold());

    let agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(15))
        .timeout_read(Duration::from_secs(60))
        .user_agent("anime4k-cli/1.0")
        .build();

    let response = agent
        .get(url)
        .call()
        .with_context(|| format!("Failed to download from URL: {}", url))?;

    let total_size = response
        .header("Content-Length")
        .and_then(|len| len.parse::<u64>().ok())
        .unwrap_or(0);

    let pb = if total_size > 0 {
        let pb = ProgressBar::new(total_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})",
                )?
                .progress_chars("━╾─"),
        );
        pb
    } else {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] {bytes} downloaded ({bytes_per_sec})")?,
        );
        pb
    };

    pb.enable_steady_tick(Duration::from_millis(80));

    let mut reader = response.into_reader();
    let mut file = File::create(output_path)
        .with_context(|| format!("Failed to create output file at {:?}", output_path))?;

    let mut buffer = [0u8; 32768];
    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])?;
        pb.inc(n as u64);
    }

    file.flush()?;
    pb.finish_with_message("Download complete");
    println!("{} Download finished successfully!", "✨".green());

    Ok(())
}

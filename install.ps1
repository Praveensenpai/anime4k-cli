# Anime4K GLSL Shaders & Hotkeys Installer for mpv (Windows PowerShell)
# Repository: https://github.com/Praveensenpai/anime4k-cli

$ErrorActionPreference = "Stop"

$Repo = "Praveensenpai/anime4k-cli"
$ReleasesUrl = "https://github.com/$Repo/releases"

Write-Host "📺 Anime4K GLSL Installer & Manager for mpv" -ForegroundColor Cyan

# Detect architecture
$Arch = if ([System.Environment]::Is64BitOperatingSystem) { "x86_64" } else { "x86" }
if ($Arch -ne "x86_64") {
    Write-Host "❌ 32-bit Windows is not supported." -ForegroundColor Red
    Exit 1
}

$InstallDir = Join-Path $env:LOCALAPPDATA "Programs\anime4k"
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

$TargetExe = Join-Path $InstallDir "anime4k.exe"
$ZipUrl = "$ReleasesUrl/latest/download/anime4k-windows-x86_64.zip"
$TempZip = Join-Path $env:TEMP "anime4k-download.zip"

Write-Host "📦 Fetching pre-built binary for Windows..." -ForegroundColor Blue

try {
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    Invoke-WebRequest -Uri $ZipUrl -OutFile $TempZip -UseBasicParsing
    Expand-Archive -Path $TempZip -DestinationPath $InstallDir -Force
    Remove-Item $TempZip -Force
} catch {
    Write-Host "⚠️ Pre-built release binary not found or download failed." -ForegroundColor Yellow
    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        Write-Host "ℹ️ Compiling from source using cargo..." -ForegroundColor Blue
        cargo install --git "https://github.com/$Repo.git" --bin anime4k
        $TargetExe = (Get-Command anime4k).Source
    } else {
        Write-Host "❌ Failed to download anime4k.exe and 'cargo' is not installed." -ForegroundColor Red
        Write-Host "Please download the binary manually from: $ReleasesUrl" -ForegroundColor Yellow
        Exit 1
    }
}

# Add to user PATH if not present
$UserPath = [System.Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [System.Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    $env:Path += ";$InstallDir"
    Write-Host "✨ Added $InstallDir to User PATH." -ForegroundColor Green
}

# Run binary
& $TargetExe @args

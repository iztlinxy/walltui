use std::path::Path;

pub fn set_wallpaper(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        set_wallpaper_windows(path)
    }
    #[cfg(target_os = "linux")]
    {
        set_wallpaper_linux(path)
    }
    #[cfg(target_os = "macos")]
    {
        set_wallpaper_macos(path)
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        Err("Unsupported platform".into())
    }
}

pub fn set_video_wallpaper(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        set_video_wallpaper_windows(path)
    }
    #[cfg(target_os = "linux")]
    {
        Err("Video wallpaper not supported on Linux".into())
    }
    #[cfg(target_os = "macos")]
    {
        Err("Video wallpaper not supported on macOS".into())
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        Err("Unsupported platform".into())
    }
}

pub fn stop_video_wallpaper() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        stop_video_wallpaper_windows()
    }
    #[cfg(target_os = "linux")]
    {
        Ok(())
    }
    #[cfg(target_os = "macos")]
    {
        Ok(())
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        Err("Unsupported platform".into())
    }
}

#[cfg(target_os = "windows")]
fn set_wallpaper_windows(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let path_str = path.to_str().ok_or("Invalid path")?;
    let script = format!(
        r#"Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public class Wallpaper {{
    [DllImport("user32.dll", CharSet = CharSet.Auto)]
    public static extern int SystemParametersInfo(int uAction, int uParam, string lpvParam, int fuWinIni);
}}
"@
[Wallpaper]::SystemParametersInfo(0x0014, 0, "{path_str}", 0x01 -bor 0x02)"#
    );
    std::process::Command::new("powershell")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(&script)
        .output()?;
    Ok(())
}

#[cfg(target_os = "linux")]
fn set_wallpaper_linux(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let path_str = path.to_str().ok_or("Invalid path")?;
    // Try gsettings (GNOME) first, then feh (i3/standalone)
    if std::process::Command::new("gsettings")
        .args([
            "set",
            "org.gnome.desktop.background",
            "picture-uri",
            &format!("file://{path_str}"),
        ])
        .output()
        .is_ok()
    {
        return Ok(());
    }
    if std::process::Command::new("feh")
        .args(["--bg-fill", path_str])
        .output()
        .is_ok()
    {
        return Ok(());
    }
    Err("No supported wallpaper setter found on Linux".into())
}

#[cfg(target_os = "macos")]
fn set_wallpaper_macos(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let path_str = path.to_str().ok_or("Invalid path")?;
    let script = format!(
        r#"tell application "System Events" to set picture of every desktop to POSIX file "{path_str}""#
    );
    std::process::Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn set_video_wallpaper_windows(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let path_str = path.to_str().ok_or("Invalid path")?;

    // Try Lively Wallpaper first (most popular Windows live wallpaper app)
    let lively_paths = [
        r"C:\Program Files\Lively Wallpaper\Lively.exe",
        r"C:\Program Files (x86)\Lively Wallpaper\Lively.exe",
    ];

    for lively_path in &lively_paths {
        if std::path::Path::new(lively_path).exists() {
            std::process::Command::new(lively_path)
                .arg("--set-wallpaper")
                .arg(path_str)
                .spawn()
                .map_err(|e| format!("Failed to start Lively Wallpaper: {e}"))?;
            return Ok(());
        }
    }

    // Fall back to mpv in borderless mode
    let mpv_result = std::process::Command::new("mpv")
        .args([
            "--no-border",
            "--no-input-default-bindings",
            "--input-vo-keyboard=no",
            "--input-media-keys=no",
            "--ontop",
            "--geometry=100%:100%",
            "--no-audio",
            "--loop=inf",
            "--wid=0",
            path_str,
        ])
        .spawn();

    match mpv_result {
        Ok(_) => Ok(()),
        Err(e) => Err(
            format!("No video wallpaper tool found. Install Lively Wallpaper (https://rocksdanister.github.io/lively/) or mpv: {e}").into()
        ),
    }
}

#[cfg(target_os = "windows")]
fn stop_video_wallpaper_windows() -> Result<(), Box<dyn std::error::Error>> {
    // Try to stop mpv instances
    let _ = std::process::Command::new("taskkill")
        .args(["/F", "/IM", "mpv.exe"])
        .output();

    // Try to stop Lively Wallpaper wallpaper
    let _ = std::process::Command::new("powershell")
        .arg("-Command")
        .arg("Get-Process -Name 'Lively' -ErrorAction SilentlyContinue | Stop-Process")
        .output();

    Ok(())
}

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

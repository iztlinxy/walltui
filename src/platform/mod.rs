use std::path::Path;

pub fn set_wallpaper(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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

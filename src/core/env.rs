use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use etc_os_release::OsRelease;

use super::backend::BackendKind;

#[derive(Debug, Clone)]
pub struct EnvInfo {
    pub os_release: BTreeMap<String, String>,
    pub kernel: String,
}

impl EnvInfo {
    pub fn detect() -> Result<Self> {
        let os_release = detect_os_release().context("не удалось определить /etc/os-release")?;
        let kernel = detect_kernel().context("не удалось определить версию ядра")?;
        Ok(Self { os_release, kernel })
    }

    pub fn id(&self) -> Option<&str> {
        self.os_release.get("ID").map(|s| s.as_str())
    }

    pub fn id_like(&self) -> Option<&str> {
        self.os_release.get("ID_LIKE").map(|s| s.as_str())
    }

    pub fn pretty_name(&self) -> Option<&str> {
        self.os_release.get("PRETTY_NAME").map(|s| s.as_str())
    }

    pub fn recommended_backend(&self) -> BackendKind {
        let id = self.id().unwrap_or_default().to_ascii_lowercase();
        let id_like = self.id_like().unwrap_or_default().to_ascii_lowercase();

        // XBPS: Void Linux
        if id == "void" || id_like.contains("void") {
            return BackendKind::Xbps;
        }

        // Pacman: Arch/Manjaro/EndeavourOS/Artix/etc.
        if id == "arch"
            || id_like.contains("arch")
            || id == "manjaro"
            || id_like.contains("manjaro")
        {
            return BackendKind::Pacman;
        }

        // APT: Debian/Ubuntu/Mint/etc. (дефолт)
        BackendKind::Apt
    }
}

fn detect_kernel() -> Result<String> {
    let out = Command::new("uname")
        .arg("-r")
        .output()
        .context("вызов uname -r")?;
    if !out.status.success() {
        return Err(anyhow!("uname -r завершился с кодом {}", out.status));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn detect_os_release() -> Result<BTreeMap<String, String>> {
    // 1) etc-os-release (если файл стандартный)
    if let Ok(o) = OsRelease::open() {
        return Ok(os_release_to_map(o));
    }

    // 2) fallback: вручную парсим /etc/os-release
    let p = Path::new("/etc/os-release");
    let s = fs::read_to_string(p).context("чтение /etc/os-release")?;
    Ok(parse_os_release(&s))
}

fn os_release_to_map(o: OsRelease) -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    m.insert("ID".into(), o.id().to_string());
    m.insert("NAME".into(), o.name().to_string());
    m.insert("PRETTY_NAME".into(), o.pretty_name().to_string());

    if let Some(it) = o.id_like() {
        let v = it.collect::<Vec<_>>().join(" ");
        if !v.trim().is_empty() {
            m.insert("ID_LIKE".into(), v);
        }
    }
    if let Some(v) = o.version_id() {
        m.insert("VERSION_ID".into(), v.to_string());
    }
    if let Some(v) = o.version() {
        m.insert("VERSION".into(), v.to_string());
    }
    if let Some(v) = o.variant_id() {
        m.insert("VARIANT_ID".into(), v.to_string());
    }
    m
}

fn parse_os_release(input: &str) -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((k, v)) = line.split_once('=') else { continue };
        let k = k.trim();
        let v = v.trim();
        let v = unquote(v);
        m.insert(k.to_string(), v);
    }
    m
}

fn unquote(s: &str) -> String {
    let bytes = s.as_bytes();
    if bytes.len() >= 2
        && ((bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[bytes.len() - 1] == b'\''))
    {
        return s[1..s.len() - 1].to_string();
    }
    s.to_string()
}

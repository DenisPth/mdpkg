use std::ffi::OsStr;
use std::process::{Command, Stdio};

use anyhow::{Context, Result, anyhow};
use serde::Deserialize;

use crate::core::BackendKind;

pub fn command_exists(cmd: &str) -> bool {
    which::which(cmd).is_ok()
}

#[derive(Debug, Default, Deserialize)]
struct Config {
    #[serde(default)]
    backend: Option<BackendKind>,
    /// Переопределение образа distrobox-контейнера по имени бэкенда
    /// (`apt`/`pacman`/`xbps`), см. `--container`.
    #[serde(default)]
    container_images: std::collections::BTreeMap<String, String>,
}

fn load_config() -> Config {
    for p in config_candidates() {
        if let Ok(s) = std::fs::read_to_string(&p)
            && let Ok(cfg) = serde_yaml::from_str::<Config>(&s)
        {
            return cfg;
        }
    }
    Config::default()
}

pub fn load_config_backend() -> Option<BackendKind> {
    load_config().backend
}

/// Образ для `--container` с учётом пользовательского оверрайда в конфиге,
/// иначе `default`.
pub fn container_image_for(kind: BackendKind, default: &str) -> String {
    load_config()
        .container_images
        .get(kind.as_str())
        .cloned()
        .unwrap_or_else(|| default.to_string())
}

/// Убеждается, что distrobox-контейнер с этим именем существует, создавая его
/// при необходимости из указанного образа.
pub fn ensure_distrobox_container(container: &str, image: &str) -> Result<()> {
    if !command_exists("distrobox") {
        return Err(anyhow!(
            "не найден `distrobox` в PATH — установи его вместе с podman или docker, \
             чтобы запускать пакетный менеджер другого дистрибутива в контейнере \
             (https://github.com/89luca89/distrobox)"
        ));
    }

    let exists = Command::new("distrobox")
        .args(["enter", container, "--", "true"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if exists {
        return Ok(());
    }

    eprintln!("Контейнер `{container}` не найден, создаю (образ: {image})...");
    run_cmd(
        "distrobox",
        ["create", "--image", image, "--name", container, "--yes"],
    )
}

fn config_candidates() -> Vec<std::path::PathBuf> {
    let mut v = Vec::new();
    // 1) ./multipkgdp.yml
    v.push(std::path::PathBuf::from("multipkgdp.yml"));

    // 2) $XDG_CONFIG_HOME/multipkgdp.yml или ~/.config/multipkgdp.yml
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        v.push(std::path::PathBuf::from(xdg).join("multipkgdp.yml"));
    } else if let Ok(home) = std::env::var("HOME") {
        v.push(std::path::PathBuf::from(home).join(".config/multipkgdp.yml"));
    }

    // 3) /etc/multipkgdp.yml
    v.push(std::path::PathBuf::from("/etc/multipkgdp.yml"));
    v
}

pub fn run_cmd<I, S>(program: &str, args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let status = Command::new(program)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("не удалось запустить `{program}`"))?;

    if !status.success() {
        return Err(anyhow!("`{program}` завершился с кодом {status}"));
    }
    Ok(())
}

pub fn run_cmd_capture_stdout<I, S>(program: &str, args: I) -> Result<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let out = Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .with_context(|| format!("не удалось запустить `{program}`"))?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(anyhow!(
            "`{program}` завершился с кодом {status}: {stderr}",
            status = out.status
        ));
    }

    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

pub fn run_cmd_sudo<I, S>(program: &str, args: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    // Если уже root — не используем sudo.
    if nix::unistd::Uid::effective().is_root() {
        return run_cmd(program, args);
    }

    let mut full: Vec<std::ffi::OsString> = Vec::new();
    full.push(program.into());
    for a in args {
        full.push(a.as_ref().to_os_string());
    }

    run_cmd("sudo", full)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_backend_parses_from_yaml() {
        let cfg: Config = serde_yaml::from_str("backend: pacman\n").unwrap();
        assert_eq!(cfg.backend, Some(BackendKind::Pacman));
    }

    #[test]
    fn config_backend_missing_field_is_none() {
        let cfg: Config = serde_yaml::from_str("something_else: true\n").unwrap();
        assert_eq!(cfg.backend, None);
    }

    #[test]
    fn config_parses_container_image_overrides() {
        let cfg: Config =
            serde_yaml::from_str("container_images:\n  xbps: my-registry/void:latest\n").unwrap();
        assert_eq!(
            cfg.container_images.get("xbps").map(String::as_str),
            Some("my-registry/void:latest")
        );
    }

    #[test]
    fn command_exists_finds_a_common_binary() {
        // `sh` should exist on any POSIX system the tests run on.
        assert!(command_exists("sh"));
        assert!(!command_exists("definitely-not-a-real-command-xyz"));
    }
}

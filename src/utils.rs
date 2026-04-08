use std::ffi::OsStr;
use std::process::{Command, Stdio};

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

use crate::core::BackendKind;

pub fn command_exists(cmd: &str) -> bool {
    which::which(cmd).is_ok()
}

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(default)]
    backend: Option<BackendKind>,
}

pub fn load_config_backend() -> Option<BackendKind> {
    let candidates = config_candidates();
    for p in candidates {
        if let Ok(s) = std::fs::read_to_string(&p) {
            if let Ok(cfg) = serde_yaml::from_str::<Config>(&s) {
                if cfg.backend.is_some() {
                    return cfg.backend;
                }
            }
        }
    }
    None
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

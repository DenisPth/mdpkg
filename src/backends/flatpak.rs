use anyhow::{Result, anyhow};

use crate::core::{Backend, BackendKind, EnvInfo};
use crate::utils::{command_exists, run_cmd};

/// Универсальный, дистро-независимый бэкенд поверх Flatpak/Flathub.
/// В отличие от apt/pacman/xbps не привязан к конкретному дистрибутиву и
/// поэтому не поддерживает (и не нуждается в) `--container`.
#[derive(Debug, Default)]
pub struct FlatpakBackend;

impl FlatpakBackend {
    pub fn new() -> Self {
        Self
    }

    fn ensure_available(&self) -> Result<()> {
        if command_exists("flatpak") {
            Ok(())
        } else {
            Err(anyhow!("не найдено `flatpak` в PATH"))
        }
    }

    fn ensure_flathub_remote(&self) -> Result<()> {
        // Идемпотентно: --if-not-exists не ругается, если remote уже есть.
        run_cmd(
            "flatpak",
            [
                "remote-add",
                "--if-not-exists",
                "flathub",
                "https://flathub.org/repo/flathub.flatpakrepo",
            ],
        )
    }
}

impl Backend for FlatpakBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::Flatpak
    }

    fn install(&self, _env: &EnvInfo, packages: &[String]) -> Result<()> {
        self.ensure_available()?;
        self.ensure_flathub_remote()?;
        // flatpak install -y flathub app.id...
        let mut args = vec![
            "install".to_string(),
            "-y".to_string(),
            "flathub".to_string(),
        ];
        args.extend(packages.iter().cloned());
        run_cmd("flatpak", args)
    }

    fn remove(&self, _env: &EnvInfo, packages: &[String], _flags: &str) -> Result<()> {
        self.ensure_available()?;
        // flatpak uninstall -y app.id...
        let mut args = vec!["uninstall".to_string(), "-y".to_string()];
        args.extend(packages.iter().cloned());
        run_cmd("flatpak", args)
    }

    fn update(&self, _env: &EnvInfo) -> Result<()> {
        self.ensure_available()?;
        run_cmd("flatpak", ["update", "-y"])
    }

    fn search(&self, _env: &EnvInfo, query: &str) -> Result<()> {
        self.ensure_available()?;
        self.ensure_flathub_remote()?;
        run_cmd("flatpak", ["search", query])
    }

    fn list(&self, _env: &EnvInfo) -> Result<()> {
        self.ensure_available()?;
        run_cmd("flatpak", ["list"])
    }
}

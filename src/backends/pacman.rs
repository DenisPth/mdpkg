use anyhow::{anyhow, Result};

use crate::core::{Backend, BackendKind, EnvInfo};
use crate::utils::{command_exists, run_cmd, run_cmd_sudo};

#[derive(Debug, Default)]
pub struct PacmanBackend;

impl PacmanBackend {
    pub fn new() -> Self {
        Self
    }

    fn ensure_available(&self) -> Result<()> {
        if command_exists("pacman") {
            Ok(())
        } else {
            Err(anyhow!("не найдено `pacman` в PATH"))
        }
    }
}

impl Backend for PacmanBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::Pacman
    }

    fn install(&self, _env: &EnvInfo, packages: &[String]) -> Result<()> {
        self.ensure_available()?;
        // pacman -S --noconfirm pkgs...
        let mut args = vec!["-S".into(), "--noconfirm".into()];
        args.extend(packages.iter().cloned());
        run_cmd_sudo("pacman", args)
    }

    fn remove(&self, _env: &EnvInfo, packages: &[String]) -> Result<()> {
        self.ensure_available()?;
        // pacman -Rns --noconfirm pkgs...
        let mut args = vec!["-Rns".into(), "--noconfirm".into()];
        args.extend(packages.iter().cloned());
        run_cmd_sudo("pacman", args)
    }

    fn update(&self, _env: &EnvInfo) -> Result<()> {
        self.ensure_available()?;
        // pacman -Syu --noconfirm
        run_cmd_sudo("pacman", ["-Syu", "--noconfirm"])
    }

    fn search(&self, _env: &EnvInfo, query: &str) -> Result<()> {
        self.ensure_available()?;
        // pacman -Ss QUERY
        run_cmd("pacman", ["-Ss", query])
    }

    fn list(&self, _env: &EnvInfo) -> Result<()> {
        self.ensure_available()?;
        // pacman -Q
        run_cmd("pacman", ["-Q"])
    }
}

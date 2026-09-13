use anyhow::{Result, anyhow};

use crate::core::{Backend, BackendKind, EnvInfo, Exec};

#[derive(Debug, Default)]
pub struct PacmanBackend {
    exec: Exec,
}

impl PacmanBackend {
    pub fn with_exec(exec: Exec) -> Self {
        Self { exec }
    }

    fn ensure_available(&self) -> Result<()> {
        if self.exec.command_exists("pacman") {
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
        self.exec.run_sudo("pacman", args)
    }

    fn remove(&self, _env: &EnvInfo, packages: &[String], flags: &str) -> Result<()> {
        self.ensure_available()?;
        // pacman -R<flags> --noconfirm pkgs... — суффикс пробрасывается как ввёл
        // пользователь (pacman нативно понимает любые комбинации n/s/c/u).
        let mut args = vec![format!("-R{flags}"), "--noconfirm".into()];
        args.extend(packages.iter().cloned());
        self.exec.run_sudo("pacman", args)
    }

    fn update(&self, _env: &EnvInfo) -> Result<()> {
        self.ensure_available()?;
        // pacman -Syu --noconfirm
        self.exec.run_sudo("pacman", ["-Syu", "--noconfirm"])
    }

    fn search(&self, _env: &EnvInfo, query: &str) -> Result<()> {
        self.ensure_available()?;
        // pacman -Ss QUERY
        self.exec.run("pacman", ["-Ss", query])
    }

    fn list(&self, _env: &EnvInfo) -> Result<()> {
        self.ensure_available()?;
        // pacman -Q
        self.exec.run("pacman", ["-Q"])
    }
}

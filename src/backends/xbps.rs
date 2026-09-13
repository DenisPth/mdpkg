use anyhow::{Result, anyhow};

use crate::core::{Backend, BackendKind, EnvInfo, Exec};

#[derive(Debug, Default)]
pub struct XbpsBackend {
    exec: Exec,
}

impl XbpsBackend {
    pub fn with_exec(exec: Exec) -> Self {
        Self { exec }
    }

    fn ensure_available(&self) -> Result<()> {
        if self.exec.command_exists("xbps-install") {
            Ok(())
        } else {
            Err(anyhow!("не найдено `xbps-install` в PATH"))
        }
    }
}

impl Backend for XbpsBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::Xbps
    }

    fn install(&self, _env: &EnvInfo, packages: &[String]) -> Result<()> {
        self.ensure_available()?;
        // xbps-install -y pkgs...
        let mut args = vec!["-y".into()];
        args.extend(packages.iter().cloned());
        self.exec.run_sudo("xbps-install", args)
    }

    fn remove(&self, _env: &EnvInfo, packages: &[String], _flags: &str) -> Result<()> {
        self.ensure_available()?;
        // xbps-remove -Ry pkgs... — xbps-remove уже всегда рекурсивно чистит
        // сироты-зависимости через -R, отдельных суффиксов как в pacman у него нет.
        let mut args = vec!["-Ry".into()];
        args.extend(packages.iter().cloned());
        self.exec.run_sudo("xbps-remove", args)
    }

    fn update(&self, _env: &EnvInfo) -> Result<()> {
        self.ensure_available()?;
        // xbps-install -Syu
        self.exec.run_sudo("xbps-install", ["-Syu"])
    }

    fn search(&self, _env: &EnvInfo, query: &str) -> Result<()> {
        // xbps-query -Rs QUERY
        if self.exec.command_exists("xbps-query") {
            self.exec.run("xbps-query", ["-Rs", query])
        } else {
            Err(anyhow!("не найдено `xbps-query` в PATH"))
        }
    }

    fn list(&self, _env: &EnvInfo) -> Result<()> {
        // xbps-query -l
        if self.exec.command_exists("xbps-query") {
            self.exec.run("xbps-query", ["-l"])
        } else {
            Err(anyhow!("не найдено `xbps-query` в PATH"))
        }
    }
}

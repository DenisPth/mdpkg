use anyhow::{anyhow, Result};

use crate::core::{Backend, BackendKind, EnvInfo};
use crate::utils::{command_exists, run_cmd, run_cmd_sudo};

#[derive(Debug, Default)]
pub struct XbpsBackend;

impl XbpsBackend {
    pub fn new() -> Self {
        Self
    }

    fn ensure_available(&self) -> Result<()> {
        if command_exists("xbps-install") {
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
        run_cmd_sudo("xbps-install", args)
    }

    fn remove(&self, _env: &EnvInfo, packages: &[String]) -> Result<()> {
        self.ensure_available()?;
        // xbps-remove -Ry pkgs...
        let mut args = vec!["-Ry".into()];
        args.extend(packages.iter().cloned());
        run_cmd_sudo("xbps-remove", args)
    }

    fn update(&self, _env: &EnvInfo) -> Result<()> {
        self.ensure_available()?;
        // xbps-install -Syu
        run_cmd_sudo("xbps-install", ["-Syu"])
    }

    fn search(&self, _env: &EnvInfo, query: &str) -> Result<()> {
        // xbps-query -Rs QUERY
        if command_exists("xbps-query") {
            run_cmd("xbps-query", ["-Rs", query])
        } else {
            Err(anyhow!("не найдено `xbps-query` в PATH"))
        }
    }

    fn list(&self, _env: &EnvInfo) -> Result<()> {
        // xbps-query -l
        if command_exists("xbps-query") {
            run_cmd("xbps-query", ["-l"])
        } else {
            Err(anyhow!("не найдено `xbps-query` в PATH"))
        }
    }
}

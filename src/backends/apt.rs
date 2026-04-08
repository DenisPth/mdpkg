use anyhow::{anyhow, Result};

use crate::core::{Backend, BackendKind, EnvInfo};
use crate::utils::{command_exists, run_cmd, run_cmd_sudo};

#[derive(Debug, Default)]
pub struct AptBackend;

impl AptBackend {
    pub fn new() -> Self {
        Self
    }

    fn ensure_available(&self) -> Result<()> {
        if command_exists("apt-get") {
            Ok(())
        } else {
            Err(anyhow!("не найдено `apt-get` в PATH"))
        }
    }
}

impl Backend for AptBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::Apt
    }

    fn install(&self, _env: &EnvInfo, packages: &[String]) -> Result<()> {
        self.ensure_available()?;
        // apt-get install -y pkgs...
        let mut args = vec!["install".into(), "-y".into()];
        args.extend(packages.iter().cloned());
        run_cmd_sudo("apt-get", args)
    }

    fn remove(&self, _env: &EnvInfo, packages: &[String]) -> Result<()> {
        self.ensure_available()?;
        // apt-get remove -y pkgs...
        let mut args = vec!["remove".into(), "-y".into()];
        args.extend(packages.iter().cloned());
        run_cmd_sudo("apt-get", args)
    }

    fn update(&self, _env: &EnvInfo) -> Result<()> {
        self.ensure_available()?;
        // apt-get update && apt-get upgrade -y
        run_cmd_sudo("apt-get", ["update"])?;
        run_cmd_sudo("apt-get", ["upgrade", "-y"])
    }

    fn search(&self, _env: &EnvInfo, query: &str) -> Result<()> {
        // apt-cache search QUERY
        if command_exists("apt-cache") {
            run_cmd("apt-cache", ["search", query])
        } else if command_exists("apt") {
            run_cmd("apt", ["search", query])
        } else {
            Err(anyhow!("не найдено `apt-cache` или `apt` в PATH"))
        }
    }

    fn list(&self, _env: &EnvInfo) -> Result<()> {
        // dpkg -l
        if command_exists("dpkg") {
            run_cmd("dpkg", ["-l"])
        } else if command_exists("apt") {
            run_cmd("apt", ["list", "--installed"])
        } else {
            Err(anyhow!("не найдено `dpkg` или `apt` в PATH"))
        }
    }
}

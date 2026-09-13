use anyhow::{Result, anyhow};

use crate::core::{Backend, BackendKind, EnvInfo, Exec};

#[derive(Debug, Default)]
pub struct AptBackend {
    exec: Exec,
}

impl AptBackend {
    pub fn with_exec(exec: Exec) -> Self {
        Self { exec }
    }

    fn ensure_available(&self) -> Result<()> {
        if self.exec.command_exists("apt-get") {
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
        self.exec.run_sudo("apt-get", args)
    }

    fn remove(&self, _env: &EnvInfo, packages: &[String], flags: &str) -> Result<()> {
        self.ensure_available()?;
        // `n` (в pacman: не сохранять бэкапы/конфиги) ближе всего к `purge` в apt.
        let subcmd = if flags.contains('n') {
            "purge"
        } else {
            "remove"
        };
        let mut args = vec![subcmd.into(), "-y".into()];
        args.extend(packages.iter().cloned());
        self.exec.run_sudo("apt-get", args)?;

        // `s` (в pacman: рекурсивно снести ненужные зависимости) — ближайший
        // аналог в apt это отдельный проход autoremove.
        if flags.contains('s') {
            self.exec.run_sudo("apt-get", ["autoremove", "-y"])?;
        }
        Ok(())
    }

    fn update(&self, _env: &EnvInfo) -> Result<()> {
        self.ensure_available()?;
        // apt-get update && apt-get upgrade -y
        self.exec.run_sudo("apt-get", ["update"])?;
        self.exec.run_sudo("apt-get", ["upgrade", "-y"])
    }

    fn search(&self, _env: &EnvInfo, query: &str) -> Result<()> {
        // apt-cache search QUERY
        if self.exec.command_exists("apt-cache") {
            self.exec.run("apt-cache", ["search", query])
        } else if self.exec.command_exists("apt") {
            self.exec.run("apt", ["search", query])
        } else {
            Err(anyhow!("не найдено `apt-cache` или `apt` в PATH"))
        }
    }

    fn list(&self, _env: &EnvInfo) -> Result<()> {
        // dpkg -l
        if self.exec.command_exists("dpkg") {
            self.exec.run("dpkg", ["-l"])
        } else if self.exec.command_exists("apt") {
            self.exec.run("apt", ["list", "--installed"])
        } else {
            Err(anyhow!("не найдено `dpkg` или `apt` в PATH"))
        }
    }
}

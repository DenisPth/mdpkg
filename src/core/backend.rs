use anyhow::Result;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

use super::env::EnvInfo;

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BackendKind {
    Apt,
    Pacman,
    Xbps,
}

impl BackendKind {
    pub fn as_str(self) -> &'static str {
        match self {
            BackendKind::Apt => "apt",
            BackendKind::Pacman => "pacman",
            BackendKind::Xbps => "xbps",
        }
    }
}

pub trait Backend: Send + Sync {
    fn kind(&self) -> BackendKind;

    fn install(&self, env: &EnvInfo, packages: &[String]) -> Result<()>;
    fn remove(&self, env: &EnvInfo, packages: &[String]) -> Result<()>;
    fn update(&self, env: &EnvInfo) -> Result<()>;
    fn search(&self, env: &EnvInfo, query: &str) -> Result<()>;
    fn list(&self, env: &EnvInfo) -> Result<()>;
}

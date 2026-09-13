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
    /// Универсальный, дистро-независимый бэкенд (Flathub).
    Flatpak,
}

impl BackendKind {
    pub fn as_str(self) -> &'static str {
        match self {
            BackendKind::Apt => "apt",
            BackendKind::Pacman => "pacman",
            BackendKind::Xbps => "xbps",
            BackendKind::Flatpak => "flatpak",
        }
    }

    /// Образ-по-умолчанию для запуска этого бэкенда внутри distrobox-контейнера
    /// (флаг `--container`) — так можно ставить, например, xbps-пакеты на Arch,
    /// не рискуя хостовой системой. `None` — контейнеризация для этого бэкенда
    /// не имеет смысла (сам по себе уже кроссдистрибутивный).
    pub fn default_container_image(self) -> Option<&'static str> {
        match self {
            BackendKind::Apt => Some("debian:stable"),
            BackendKind::Pacman => Some("archlinux:latest"),
            BackendKind::Xbps => Some("ghcr.io/void-linux/void-glibc-busybox:latest"),
            BackendKind::Flatpak => None,
        }
    }

    /// Имя persistent-контейнера distrobox, используемого для этого бэкенда.
    pub fn container_name(self) -> String {
        format!("mdpkg-{}", self.as_str())
    }
}

pub trait Backend: Send + Sync {
    fn kind(&self) -> BackendKind;

    fn install(&self, env: &EnvInfo, packages: &[String]) -> Result<()>;
    /// `flags` — суффикс из `-R<flags>` как ввёл пользователь (например `"ns"` для `-Rns`,
    /// пустая строка для голого `-R`). Бэкенды вольны использовать его как имеет смысл
    /// для своего пакетного менеджера (см. реализации в `backends/`).
    fn remove(&self, env: &EnvInfo, packages: &[String], flags: &str) -> Result<()>;
    fn update(&self, env: &EnvInfo) -> Result<()>;
    fn search(&self, env: &EnvInfo, query: &str) -> Result<()>;
    fn list(&self, env: &EnvInfo) -> Result<()>;
}

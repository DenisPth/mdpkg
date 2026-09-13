mod apt;
mod flatpak;
mod pacman;
mod xbps;

use crate::core::{Backend, BackendKind, Exec};

pub use apt::AptBackend;
pub use flatpak::FlatpakBackend;
pub use pacman::PacmanBackend;
pub use xbps::XbpsBackend;

pub fn make_backend(kind: BackendKind, exec: Exec) -> Box<dyn Backend> {
    match kind {
        BackendKind::Apt => Box::new(AptBackend::with_exec(exec)),
        BackendKind::Pacman => Box::new(PacmanBackend::with_exec(exec)),
        BackendKind::Xbps => Box::new(XbpsBackend::with_exec(exec)),
        // Flatpak сам по себе кроссдистрибутивный, контейнер ему не нужен.
        BackendKind::Flatpak => Box::new(FlatpakBackend::new()),
    }
}

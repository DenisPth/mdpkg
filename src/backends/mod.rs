mod apt;
mod pacman;
mod xbps;

use crate::core::{Backend, BackendKind};

pub use apt::AptBackend;
pub use pacman::PacmanBackend;
pub use xbps::XbpsBackend;

pub fn make_backend(kind: BackendKind) -> Box<dyn Backend> {
    match kind {
        BackendKind::Apt => Box::new(AptBackend::new()),
        BackendKind::Pacman => Box::new(PacmanBackend::new()),
        BackendKind::Xbps => Box::new(XbpsBackend::new()),
    }
}

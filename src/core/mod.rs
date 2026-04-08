pub mod action;
pub mod package;
pub mod backend;
pub mod env;

pub use action::Action;
pub use backend::{Backend, BackendKind};
pub use env::EnvInfo;

use anyhow::Result;

pub fn run(backend: &dyn Backend, env: &EnvInfo, action: Action) -> Result<()> {
    match action {
        Action::Install { packages } => backend.install(env, &packages),
        Action::Remove { packages } => backend.remove(env, &packages),
        Action::Update => backend.update(env),
        Action::Search { query } => backend.search(env, &query),
        Action::List => backend.list(env),
    }
}
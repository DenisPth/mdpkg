pub mod action;
pub mod backend;
pub mod env;

pub use action::Action;
pub use backend::{Backend, BackendKind};
pub use env::EnvInfo;

use anyhow::Result;

fn print_env_banner(backend: &dyn Backend, env: &EnvInfo) {
    eprintln!(
        "[{}] OS: {} (kernel {}) | Backend: {}",
        env!("CARGO_PKG_NAME"),
        env.pretty_name().unwrap_or("неизвестно"),
        env.kernel,
        backend.kind().as_str(),
    );
}

pub fn run(backend: &dyn Backend, env: &EnvInfo, action: Action) -> Result<()> {
    match action {
        Action::Install { packages } => {
            print_env_banner(backend, env);
            backend.install(env, &packages)
        }
        Action::Remove { packages, flags } => {
            print_env_banner(backend, env);
            backend.remove(env, &packages, &flags)
        }
        Action::Update => {
            print_env_banner(backend, env);
            backend.update(env)
        }
        Action::Search { query } => backend.search(env, &query),
        Action::List => backend.list(env),
    }
}

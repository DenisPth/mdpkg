use anyhow::Result;

use crate::utils;

/// Где на самом деле выполняются команды бэкенда: локально на хосте, или внутри
/// distrobox-контейнера — это то, что позволяет запускать "чужой" пакетный
/// менеджер (например xbps на Arch) без риска сломать хостовую систему.
#[derive(Debug, Clone, Default)]
pub enum Exec {
    #[default]
    Local,
    Distrobox {
        container: String,
    },
}

impl Exec {
    fn wrap(&self, program: &str, args: Vec<String>) -> (String, Vec<String>) {
        match self {
            Exec::Local => (program.to_string(), args),
            Exec::Distrobox { container } => {
                let mut full = vec![
                    "enter".to_string(),
                    container.clone(),
                    "--".to_string(),
                    program.to_string(),
                ];
                full.extend(args);
                ("distrobox".to_string(), full)
            }
        }
    }

    pub fn run<I, S>(&self, program: &str, args: I) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let (p, a) = self.wrap(
            program,
            args.into_iter().map(|s| s.as_ref().to_string()).collect(),
        );
        utils::run_cmd(&p, a)
    }

    pub fn run_sudo<I, S>(&self, program: &str, args: I) -> Result<()>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let args: Vec<String> = args.into_iter().map(|s| s.as_ref().to_string()).collect();
        match self {
            // На хосте — обычная логика с sudo/root.
            Exec::Local => utils::run_cmd_sudo(program, args),
            // Внутри distrobox-контейнера root не нужен так же аккуратно, как на
            // хосте: контейнер создаётся под текущим пользователем и distrobox
            // сам настраивает там passwordless sudo для этого пользователя.
            Exec::Distrobox { .. } => {
                let mut sudo_args = vec!["sudo".to_string(), program.to_string()];
                sudo_args.extend(args);
                let (p, a) = self.wrap_raw(sudo_args);
                utils::run_cmd(&p, a)
            }
        }
    }

    fn wrap_raw(&self, args: Vec<String>) -> (String, Vec<String>) {
        match self {
            Exec::Local => unreachable!("wrap_raw используется только для Distrobox"),
            Exec::Distrobox { container } => {
                let mut full = vec!["enter".to_string(), container.clone(), "--".to_string()];
                full.extend(args);
                ("distrobox".to_string(), full)
            }
        }
    }

    pub fn capture_stdout<I, S>(&self, program: &str, args: I) -> Result<String>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let (p, a) = self.wrap(
            program,
            args.into_iter().map(|s| s.as_ref().to_string()).collect(),
        );
        utils::run_cmd_capture_stdout(&p, a)
    }

    pub fn command_exists(&self, cmd: &str) -> bool {
        match self {
            Exec::Local => utils::command_exists(cmd),
            Exec::Distrobox { .. } => {
                let probe = format!("command -v {cmd}");
                self.capture_stdout("sh", ["-c", probe.as_str()]).is_ok()
            }
        }
    }
}

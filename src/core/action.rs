use anyhow::{Result, anyhow};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Установить один или несколько пакетов
    Install { packages: Vec<String> },
    /// Удалить один или несколько пакетов.
    /// `flags` — суффикс операции (например `"ns"` для `-Rns`).
    Remove {
        packages: Vec<String>,
        flags: String,
    },
    /// Обновить индексы/систему (в зависимости от бэкенда)
    Update,
    /// Поиск пакета
    Search { query: String },
    /// Показать установленные пакеты
    List,
}

/// Разбирает pacman-style операцию (`-S`, `-R...`, `-Ss`, `-Syu`, `-Q`) и её аргументы в [`Action`].
/// Вынесено из `main.rs`, чтобы логику можно было покрыть тестами без запуска всего CLI.
pub fn parse_action(op: &str, op_args: Vec<String>) -> Result<Action> {
    match op {
        "-S" => {
            if op_args.is_empty() {
                return Err(anyhow!("ожидался хотя бы один пакет после `-S`"));
            }
            Ok(Action::Install { packages: op_args })
        }
        op if op.starts_with("-R") => {
            if op_args.is_empty() {
                return Err(anyhow!("ожидался хотя бы один пакет после `{op}`"));
            }
            Ok(Action::Remove {
                packages: op_args,
                flags: op.trim_start_matches("-R").to_string(),
            })
        }
        "-Ss" => {
            let query = op_args
                .first()
                .ok_or_else(|| anyhow!("ожидался запрос после `-Ss`"))?
                .to_string();
            Ok(Action::Search { query })
        }
        "-Syu" => Ok(Action::Update),
        "-Q" => Ok(Action::List),
        _ => Err(anyhow!(
            "неизвестная операция `{}`. Поддержано: `-S`, `-R...` (например `-Rns`), `-Ss`, `-Syu`, `-Q`",
            op
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn install_requires_packages() {
        assert!(parse_action("-S", vec![]).is_err());
    }

    #[test]
    fn install_parses_packages() {
        let a = parse_action("-S", vec!["firefox".into(), "neovim".into()]).unwrap();
        assert_eq!(
            a,
            Action::Install {
                packages: vec!["firefox".into(), "neovim".into()]
            }
        );
    }

    #[test]
    fn remove_captures_flags_suffix() {
        let a = parse_action("-Rns", vec!["firefox".into()]).unwrap();
        assert_eq!(
            a,
            Action::Remove {
                packages: vec!["firefox".into()],
                flags: "ns".into()
            }
        );
    }

    #[test]
    fn bare_remove_has_empty_flags() {
        let a = parse_action("-R", vec!["firefox".into()]).unwrap();
        assert_eq!(
            a,
            Action::Remove {
                packages: vec!["firefox".into()],
                flags: "".into()
            }
        );
    }

    #[test]
    fn remove_requires_packages() {
        assert!(parse_action("-Rns", vec![]).is_err());
    }

    #[test]
    fn search_requires_query() {
        assert!(parse_action("-Ss", vec![]).is_err());
    }

    #[test]
    fn search_parses_query() {
        let a = parse_action("-Ss", vec!["firefox".into()]).unwrap();
        assert_eq!(
            a,
            Action::Search {
                query: "firefox".into()
            }
        );
    }

    #[test]
    fn update_and_list() {
        assert_eq!(parse_action("-Syu", vec![]).unwrap(), Action::Update);
        assert_eq!(parse_action("-Q", vec![]).unwrap(), Action::List);
    }

    #[test]
    fn unknown_op_is_error() {
        assert!(parse_action("-X", vec![]).is_err());
    }
}

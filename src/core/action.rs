use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum Action {
    /// Установить один или несколько пакетов
    Install {
        #[arg(required = true)]
        packages: Vec<String>,
    },
    /// Удалить один или несколько пакетов
    Remove {
        #[arg(required = true)]
        packages: Vec<String>,
    },
    /// Обновить индексы/систему (в зависимости от бэкенда)
    Update,
    /// Поиск пакета
    Search { query: String },
    /// Показать установленные пакеты
    List,
}

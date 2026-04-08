mod backends;
mod core;
mod utils;

use anyhow::{anyhow, Result};
use crate::core::{Action, BackendKind, EnvInfo};
use std::io::{IsTerminal, Write};

fn print_help(bin: &str) {
    eprintln!(
        r#"Использование:
  {bin} [--backend <apt|pacman|xbps>] <OP> [ARGS...]

Где OP — pacman-style операция:
  -S <pkg...>         установить пакет(ы)
  -R... <pkg...>      удалить пакет(ы) (например: -Rns)
  -Ss <query>         поиск в репозиториях
  -Syu                обновление системы/индексов (зависит от бэкенда)
  -Q                  список установленных пакетов

Примеры:
  {bin} -S firefox
  {bin} -Rns firefox
  {bin} -Ss firefox
  {bin} -Syu
  {bin} -Q
  {bin} --backend apt -Ss firefox
"#
    );
}

fn main() -> Result<()> {
    let mut backend_override: Option<BackendKind> = None;
    let mut op: Option<String> = None;
    let mut op_args: Vec<String> = Vec::new();

    let mut it = std::env::args();
    let bin = it.next().unwrap_or_else(|| "multipkgdp".to_string());

    while let Some(a) = it.next() {
        if a == "--help" || a == "-h" {
            print_help(&bin);
            return Ok(());
        }

        if a == "--backend" {
            let v = it
                .next()
                .ok_or_else(|| anyhow!("ожидалось значение после `--backend`"))?;
            backend_override = Some(parse_backend_kind(&v)?);
            continue;
        }
        if let Some(v) = a.strip_prefix("--backend=") {
            backend_override = Some(parse_backend_kind(v)?);
            continue;
        }

        if op.is_none() && a.starts_with('-') {
            op = Some(a);
            continue;
        }

        op_args.push(a);
    }

    let op = op.ok_or_else(|| {
        print_help(&bin);
        anyhow!("не указана операция (например `-S` или `-Rns`)")
    })?;

    let env = EnvInfo::detect()?;

    let op = op.as_str();
    let action = match op {
        "-S" => {
            if op_args.is_empty() {
                return Err(anyhow!("ожидался хотя бы один пакет после `-S`"));
            }
            Action::Install { packages: op_args }
        }
        op if op.starts_with("-R") => {
            // Принимаем любые варианты `-R...` как remove. Семантику (n/s/...) можно
            // расширить позже, когда появятся флаги удаления в Backend API.
            if op_args.is_empty() {
                return Err(anyhow!("ожидался хотя бы один пакет после `{op}`"));
            }
            Action::Remove { packages: op_args }
        }
        "-Ss" => {
            let query = op_args
                .first()
                .ok_or_else(|| anyhow!("ожидался запрос после `-Ss`"))?
                .to_string();
            Action::Search { query }
        }
        "-Syu" => Action::Update,
        "-Q" => Action::List,
        _ => {
            return Err(anyhow!(
                "неизвестная операция `{}`. Поддержано: `-S`, `-R...` (например `-Rns`), `-Ss`, `-Syu`, `-Q`",
                op
            ))
        }
    };

    // Если пользователь не указал --backend и делает `-S <onepkg>`,
    // сначала поищем пакет по имени в бэкендах и предложим выбрать.
    let backend_kind = if backend_override.is_none()
        && matches!(action, Action::Install { .. })
        && is_single_package_install(&action)
        && interactive_input_available()
    {
        let pkg = single_package_name(&action).expect("checked above");
        choose_backend_interactive(&env, pkg)?
    } else {
        backend_override
            .or_else(utils::load_config_backend)
            .unwrap_or_else(|| env.recommended_backend())
    };

    let backend = backends::make_backend(backend_kind);
    core::run(backend.as_ref(), &env, action)?;
    Ok(())
}

fn parse_backend_kind(s: &str) -> Result<BackendKind> {
    match s.to_ascii_lowercase().as_str() {
        "apt" => Ok(BackendKind::Apt),
        "pacman" => Ok(BackendKind::Pacman),
        "xbps" => Ok(BackendKind::Xbps),
        _ => Err(anyhow!(
            "неизвестный бэкенд `{}` (ожидалось: apt|pacman|xbps)",
            s
        )),
    }
}

fn interactive_input_available() -> bool {
    if std::io::stdin().is_terminal() {
        return true;
    }
    // Иногда stdin не TTY (редирект/IDE), но /dev/tty доступен.
    std::fs::OpenOptions::new().read(true).open("/dev/tty").is_ok()
}

fn is_single_package_install(action: &Action) -> bool {
    match action {
        Action::Install { packages } => packages.len() == 1,
        _ => false,
    }
}

fn single_package_name<'a>(action: &'a Action) -> Option<&'a str> {
    match action {
        Action::Install { packages } => packages.first().map(|s| s.as_str()),
        _ => None,
    }
}

fn backend_available(kind: BackendKind) -> bool {
    match kind {
        BackendKind::Apt => utils::command_exists("apt-get") || utils::command_exists("apt"),
        BackendKind::Pacman => utils::command_exists("pacman"),
        BackendKind::Xbps => utils::command_exists("xbps-install"),
    }
}

fn search_preview(kind: BackendKind, query: &str) -> Result<Vec<String>> {
    let out = match kind {
        BackendKind::Pacman => {
            if !utils::command_exists("pacman") {
                return Err(anyhow!("не найдено `pacman` в PATH"));
            }
            utils::run_cmd_capture_stdout("pacman", ["-Ss", query])?
        }
        BackendKind::Apt => {
            if utils::command_exists("apt-cache") {
                utils::run_cmd_capture_stdout("apt-cache", ["search", query])?
            } else if utils::command_exists("apt") {
                utils::run_cmd_capture_stdout("apt", ["search", query])?
            } else {
                return Err(anyhow!("не найдено `apt-cache` или `apt` в PATH"));
            }
        }
        BackendKind::Xbps => {
            if !utils::command_exists("xbps-query") {
                return Err(anyhow!("не найдено `xbps-query` в PATH"));
            }
            utils::run_cmd_capture_stdout("xbps-query", ["-Rs", query])?
        }
    };

    let mut lines: Vec<String> = out
        .lines()
        .map(|l| l.trim_end().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    if kind == BackendKind::Apt {
        lines.retain(|l| !l.starts_with("Sorting") && !l.starts_with("Full Text Search"));
    }

    Ok(lines)
}

fn try_bootstrap_backend_tools(selected: BackendKind, env: &EnvInfo) -> Result<()> {
    if backend_available(selected) {
        return Ok(());
    }

    // Авто-установка “чужих” менеджеров пакетов имеет смысл только если мы можем
    // поставить утилиты через текущий менеджер.
    let host = env.recommended_backend();
    if host != BackendKind::Pacman || !backend_available(BackendKind::Pacman) {
        return Err(anyhow!(
            "бэкенд `{}` недоступен: нужные утилиты не найдены в PATH (авто-установка поддержана только на Arch через pacman)",
            selected.as_str()
        ));
    }

    let pkgs: &[&str] = match selected {
        BackendKind::Apt => &["apt", "dpkg"],
        BackendKind::Xbps => &["xbps"],
        BackendKind::Pacman => &[],
    };
    if pkgs.is_empty() {
        return Ok(());
    }

    eprintln!(
        "Не найдено `{}`. Пробую установить через pacman: {}",
        match selected {
            BackendKind::Apt => "apt-get/apt",
            BackendKind::Xbps => "xbps-install",
            BackendKind::Pacman => "pacman",
        },
        pkgs.join(" ")
    );

    // pacman -S --needed --noconfirm pkgs...
    let mut args: Vec<String> = vec!["-S".into(), "--needed".into(), "--noconfirm".into()];
    args.extend(pkgs.iter().map(|s| s.to_string()));
    utils::run_cmd_sudo("pacman", args)?;

    if backend_available(selected) {
        Ok(())
    } else {
        Err(anyhow!(
            "после установки пакетов утилиты для `{}` всё ещё не найдены в PATH",
            selected.as_str()
        ))
    }
}

fn choose_backend_interactive(env: &EnvInfo, pkg: &str) -> Result<BackendKind> {
    let default = env.recommended_backend();

    eprintln!("Поиск пакета по имени: `{}`", pkg);
    eprintln!("(бэкенды помечены как [не установлен], если утилит нет в PATH)");

    let mut pacman_hits: Option<usize> = None;
    let mut apt_hits: Option<usize> = None;
    let mut xbps_hits: Option<usize> = None;

    if backend_available(BackendKind::Pacman) {
        if let Ok(lines) = search_preview(BackendKind::Pacman, pkg) {
            pacman_hits = Some(lines.len());
        }
    }
    if backend_available(BackendKind::Apt) {
        if let Ok(lines) = search_preview(BackendKind::Apt, pkg) {
            apt_hits = Some(lines.len());
        }
    }
    if backend_available(BackendKind::Xbps) {
        if let Ok(lines) = search_preview(BackendKind::Xbps, pkg) {
            xbps_hits = Some(lines.len());
        }
    }

    eprintln!("\nВыбери репозиторий/дистрибутив (бэкенд) для установки:");
    eprintln!(
        "  1) Arch (pacman){}{}{}",
        if backend_available(BackendKind::Pacman) {
            ""
        } else {
            " [не установлен]"
        },
        match pacman_hits {
            Some(n) => format!(" [найдено: {n}]"),
            None => String::new(),
        },
        if default == BackendKind::Pacman {
            " [по умолчанию]"
        } else {
            ""
        }
    );
    eprintln!(
        "  2) Debian/Ubuntu (apt){}{}{}",
        if backend_available(BackendKind::Apt) {
            ""
        } else {
            " [не установлен]"
        },
        match apt_hits {
            Some(n) => format!(" [найдено: {n}]"),
            None => String::new(),
        },
        if default == BackendKind::Apt {
            " [по умолчанию]"
        } else {
            ""
        }
    );
    eprintln!(
        "  3) Void (xbps){}{}{}",
        if backend_available(BackendKind::Xbps) {
            ""
        } else {
            " [не установлен]"
        },
        match xbps_hits {
            Some(n) => format!(" [найдено: {n}]"),
            None => String::new(),
        },
        if default == BackendKind::Xbps {
            " [по умолчанию]"
        } else {
            ""
        }
    );

    eprint!("Введи цифру (Enter = по умолчанию): ");
    std::io::stderr().flush().ok();

    let mut line = String::new();
    read_line_interactive(&mut line)?;
    let s = line.trim();

    let chosen = match s {
        "" => default,
        "1" => BackendKind::Pacman,
        "2" => BackendKind::Apt,
        "3" => BackendKind::Xbps,
        _ => return Err(anyhow!("ожидалась цифра 1/2/3 или Enter")),
    };

    // Если утилиты отсутствуют — попробуем поставить автоматически (best-effort).
    if !backend_available(chosen) {
        try_bootstrap_backend_tools(chosen, env)?;
    }

    Ok(chosen)
}

fn read_line_interactive(buf: &mut String) -> Result<()> {
    buf.clear();
    if std::io::stdin().is_terminal() {
        std::io::stdin().read_line(buf)?;
        return Ok(());
    }

    let mut tty = std::fs::OpenOptions::new()
        .read(true)
        .open("/dev/tty")
        .map_err(|e| anyhow!("не удалось открыть /dev/tty для ввода: {e}"))?;

    use std::io::Read;
    // Читаем до '\n' простым способом.
    let mut bytes = Vec::new();
    let mut one = [0u8; 1];
    while tty.read(&mut one)? == 1 {
        bytes.push(one[0]);
        if one[0] == b'\n' {
            break;
        }
    }
    *buf = String::from_utf8_lossy(&bytes).to_string();
    Ok(())
}
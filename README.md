# multipkgdp — Multi-Backend Package Manager / Мультипакетный менеджер

**multipkgdp** — это универсальный CLI‑пакетный менеджер для Linux, который умеет работать с разными backend'ами: `apt` (Debian/Ubuntu), `pacman` (Arch), `xbps` (Void) и потенциально другими.  
Он даёт единый интерфейс команд (`install`, `remove`, `update`, `search`, `list`) независимо от дистрибутива.

---

## 🇬🇧 English

**multipkgdp** is a multi‑backend package manager for Linux written in Rust.  
It provides a unified CLI over system package managers such as:

- `apt` (Debian, Ubuntu)  
- `pacman` (Arch Linux)  
- `xbps` (Void Linux)  
- and others (planned)

### Features

- Auto‑detect OS and choose backend (`apt`, `pacman`, `xbps`).  
- Manual backend override with `--backend`.  
- Common commands: `install`, `remove`, `update`, `search`, `list`.  
- Show OS info and used repo on each install.

### Installation (from sources)

```bash
git clone https://github.com/DenisPth/mdpkg.git
cd mdpkg
cargo build --release
sudo cp target/release/multipkgdp /usr/local/bin/
```

### Usage

```bash
multipkgdp install firefox neovim
multipkgdp remove firefox
multipkgdp update
multipkgdp search firefox
multipkgdp list
multipkgdp --backend pacman install firefox
```

---

## 🇷🇺 Русский

**multipkgdp** — мультипакетный менеджер для Linux на Rust, который умеет работать с несколькими backend'ами:

- `apt` (Debian, Ubuntu)  
- `pacman` (Arch Linux)  
- `xbps` (Void Linux)  
- и др. (в планах)

### Возможности

- Автоопределение дистрибутива и выбор backend'а (`apt`, `pacman`, `xbps`).  
- Явное указание backend'а через `--backend`.  
- Единый CLI: `install`, `remove`, `update`, `search`, `list`.  
- Вывод информации об ОС и используемом репозитории при установке.

### Установка (из исходников)

```bash
git clone https://github.com/DenisPth/mdpkg.git
cd mdpkg
cargo build --release
sudo cp target/release/multipkgdp /usr/local/bin/
```

### Использование

```bash
multipkgdp install firefox neovim
multipkgdp remove firefox
multipkgdp update
multipkgdp search firefox
multipkgdp list
multipkgdp --backend pacman install firefox
```

---

## 🇺🇸 Short card (for GitHub)

`multipkgdp` is a Rust-written multi-backend package manager for Linux (apt, pacman, xbps, etc.) with a unified CLI, OS auto‑detection and backend choice.

This is the foundation for `mdpkg` project under `DenisPth` on GitHub.

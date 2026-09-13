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
- Manual backend override with `--backend` (must come before the operation).  
- pacman‑style commands: `-S`, `-R...`, `-Ss`, `-Syu`, `-Q`.  
- Shows OS info and the backend in use on `install`/`remove`/`update`.
- `-R<flags>` forwards the suffix to the backend: exact pass‑through on `pacman`
  (e.g. `-Rns` → `pacman -Rns`); on `apt`, `n` maps to `purge` and `s` triggers
  an extra `autoremove` pass; `xbps` always does a recursive removal regardless
  of the suffix.

### Installation (from sources)

```bash
git clone https://github.com/DenisPth/mdpkg.git
cd mdpkg
./install.sh                # builds a release binary and installs it,
                             # plus `mdpkg`/`mpdpg` symlinks, into /usr/local/bin
```

### Usage

```bash
multipkgdp -S firefox neovim
multipkgdp -Rns firefox
multipkgdp -Syu
multipkgdp -Ss firefox
multipkgdp -Q
multipkgdp --backend pacman -S firefox
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
- Явное указание backend'а через `--backend` (должен стоять перед операцией).  
- Pacman-style команды: `-S`, `-R...`, `-Ss`, `-Syu`, `-Q`.  
- Вывод информации об ОС и используемом бэкенде при `install`/`remove`/`update`.
- `-R<флаги>` пробрасывается в бэкенд: на `pacman` — один в один (например
  `-Rns` → `pacman -Rns`); на `apt` — `n` превращается в `purge`, а `s`
  запускает дополнительный проход `autoremove`; `xbps` всегда делает
  рекурсивное удаление независимо от суффикса.

### Установка (из исходников)

```bash
git clone https://github.com/DenisPth/mdpkg.git
cd mdpkg
./install.sh                # соберёт release-бинарник и поставит его,
                             # а также симлинки `mdpkg`/`mpdpg`, в /usr/local/bin
```

### Использование

```bash
multipkgdp -S firefox neovim
multipkgdp -Rns firefox
multipkgdp -Syu
multipkgdp -Ss firefox
multipkgdp -Q
multipkgdp --backend pacman -S firefox
```

---

## 🇺🇸 Short card (for GitHub)

`multipkgdp` is a Rust-written multi-backend package manager for Linux (apt, pacman, xbps, etc.) with a unified CLI, OS auto‑detection and backend choice.

This is the foundation for `mdpkg` project under `DenisPth` on GitHub.

---

## License

MIT — see [LICENSE](LICENSE).

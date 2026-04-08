# multipkgdp / mdpkg

Minimal “pacman-style” package CLI with multiple backends.
Commands look like `pacman` (`-S`, `-Rns`, `-Ss`, `-Syu`, `-Q`), but the actual work is delegated to whatever package manager tools are available on your system: **pacman / apt / xbps**.

> Important: this project does not “bring foreign repositories” to your distro by itself.  
> It calls **real tools** installed on your system. For “Debian on Arch” you need a container/chroot (can be added later).

## Features

- `mdpkg -S <pkg...>`: install package(s)
- `mdpkg -Rns <pkg...>`: remove package(s) (accepts any `-R...` form)
- `mdpkg -Ss <query>`: search repositories
- `mdpkg -Syu`: update (depends on backend)
- `mdpkg -Q`: list installed packages
- `mdpkg --backend <apt|pacman|xbps> ...`: force a backend
- With `mdpkg -S <single_package>` and no `--backend`: it first searches by name and then prompts you to pick a backend.

## Installation

### Requirements

- Rust (via `rustup` or your distro packages)
- `cargo`

### Quick install via script

```bash
./install.sh
```

By default it installs binaries into `/usr/local/bin`:
- `mdpkg`
- `mpdpg`
- `multipkgdp`

Install to a user prefix:

```bash
./install.sh --prefix "$HOME/.local"
```

Uninstall:

```bash
./install.sh --uninstall
```

## Examples

Search (no sudo):

```bash
mdpkg -Ss firefox
```

Install (requires sudo):

```bash
mdpkg -S firefox
```

Remove:

```bash
mdpkg -Rns firefox
```

Force a specific backend for search:

```bash
mdpkg --backend pacman -Ss firefox
```

## Notes

- If you get an error like “`sudo: ... a terminal is required`”, run the command from a normal terminal (TTY) or run `sudo -v` first.
- `apt` and `xbps` backends only make sense if those tools are actually installed and configured on your system.


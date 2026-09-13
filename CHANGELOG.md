# Changelog

## v1.0.0

First stable release. Highlights since the initial `0.1.0` prototype:

### Added
- **Unified search**: `-Ss <query>` with no explicit `--backend` now searches
  every installed backend (apt/pacman/xbps/flatpak) and prints each result
  set under a labeled header, instead of only checking one.
- **`flatpak` backend**: a genuinely distro-independent option via Flathub,
  working the same on any distro that has `flatpak` installed.
- **`--container`**: runs the `apt`/`pacman`/`xbps` backend inside an
  auto-created [distrobox](https://github.com/89luca89/distrobox) container
  instead of the host — lets you install packages from a *different*
  distro's package manager (e.g. `xbps` packages on Arch) without touching
  the host system. Container base images are configurable per backend via
  `multipkgdp.yml`.
- **Shell completions**: `--generate-completions <bash|zsh|fish|elvish|powershell>`.
- `clap`-based argument parsing (`--backend`, `--help`, `--version`),
  replacing hand-rolled argv parsing.
- CI (build/test/clippy) and an automated release pipeline: pushing a
  `vX.Y.Z` tag builds and publishes a GitHub Release.
- Unit test coverage for action parsing, OS-release parsing, backend
  selection, and config loading.
- MIT `LICENSE`.

### Fixed
- `-R<flags>` (e.g. `-Rns`) now actually affects backend behavior instead of
  being silently normalized to one hardcoded invocation: exact pass-through
  on `pacman`, `n`→`purge`/`s`→`autoremove` on `apt`.
- The "show OS info and used backend" behavior the README always advertised
  is now actually implemented.

### Changed
- Single `multipkgdp` binary with `mdpkg`/`mpdpg` installed as symlinks
  (`install.sh`), instead of three duplicate compiled binaries.

### Removed
- ~51MB of an accidentally committed `.cargo` registry cache and dead
  duplicate source files left over from a branch merge.
- Unused `Package` struct.

## v0.1.0

Initial prototype: `apt`/`pacman`/`xbps` backends behind a pacman-style CLI
with hand-rolled argument parsing.

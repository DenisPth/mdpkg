#!/usr/bin/env sh
set -eu

REPO_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

usage() {
  cat <<'EOF'
Установщик mdpkg (Rust).

Использование:
  ./install.sh              # собрать release и установить в /usr/local/bin
  ./install.sh --uninstall  # удалить из /usr/local/bin
  ./install.sh --prefix DIR # поставить в DIR/bin (по умолчанию /usr/local)

Примеры:
  ./install.sh
  ./install.sh --prefix "$HOME/.local"
  ./install.sh --uninstall
EOF
}

PREFIX="/usr/local"
MODE="install"

while [ $# -gt 0 ]; do
  case "$1" in
    -h|--help) usage; exit 0 ;;
    --uninstall) MODE="uninstall" ;;
    --prefix)
      shift
      [ $# -gt 0 ] || { echo "Ошибка: --prefix требует значение" >&2; exit 2; }
      PREFIX="$1"
      ;;
    *)
      echo "Неизвестный аргумент: $1" >&2
      echo "Используй --help" >&2
      exit 2
      ;;
  esac
  shift
done

BIN_DIR="$PREFIX/bin"

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || { echo "Не найдено: $1" >&2; exit 1; }
}

need_cmd cargo

if [ "$MODE" = "uninstall" ]; then
  echo "Удаляю из $BIN_DIR: mdpkg mpdpg multipkgdp"
  if [ "$(id -u)" -eq 0 ] || [ -w "$BIN_DIR" ] 2>/dev/null; then
    rm -f "$BIN_DIR/mdpkg" "$BIN_DIR/mpdpg" "$BIN_DIR/multipkgdp"
  else
    sudo rm -f "$BIN_DIR/mdpkg" "$BIN_DIR/mpdpg" "$BIN_DIR/multipkgdp"
  fi
  echo "Готово."
  exit 0
fi

echo "Собираю release..."
cd "$REPO_DIR"
cargo build --release

echo "Устанавливаю в $BIN_DIR..."
if [ "$(id -u)" -eq 0 ] || [ -w "$BIN_DIR" ] 2>/dev/null; then
  install -Dm755 "./target/release/mdpkg" "$BIN_DIR/mdpkg"
  install -Dm755 "./target/release/mpdpg" "$BIN_DIR/mpdpg"
  install -Dm755 "./target/release/multipkgdp" "$BIN_DIR/multipkgdp"
else
  sudo install -Dm755 "./target/release/mdpkg" "$BIN_DIR/mdpkg"
  sudo install -Dm755 "./target/release/mpdpg" "$BIN_DIR/mpdpg"
  sudo install -Dm755 "./target/release/multipkgdp" "$BIN_DIR/multipkgdp"
fi

echo "Проверка:"
command -v mdpkg >/dev/null 2>&1 && echo "  mdpkg -> $(command -v mdpkg)" || true
echo "Готово. Попробуй: mdpkg -Ss firefox"


#!/bin/sh
set -e

REPO="wuilmerj24/ludus"
BINARY_NAME="ludus"
INSTALL_DIR="$HOME/.local/bin"

detect_os() {
  if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS=$ID
  else
    echo "No se pudo detectar la distro"
    exit 1
  fi
}

get_latest_version() {
  echo "Consultando última versión..."

  LATEST=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" \
    | grep '"tag_name":' \
    | sed -E 's/.*"([^"]+)".*/\1/')

  if [ -z "$LATEST" ]; then
    echo "No se pudo obtener la versión"
    exit 1
  fi

  echo "Última versión: $LATEST"
}

get_asset_url() {
  PATTERN="$1"

  curl -s "https://api.github.com/repos/$REPO/releases/latest" \
    | grep browser_download_url \
    | cut -d '"' -f 4 \
    | grep "$PATTERN" \
    | head -n 1
}

install_deb() {
  echo "Instalando paquete .deb..."

  URL=$(get_asset_url '\.deb$')

  if [ -z "$URL" ]; then
    echo "No se encontró paquete .deb"
    exit 1
  fi

  echo "Descargando:"
  echo "$URL"

  TMP="/tmp/ludus.deb"

  curl -fL -o "$TMP" "$URL"

  sudo dpkg -i "$TMP" || sudo apt-get install -f -y

  rm -f "$TMP"

  echo "Instalación completada"
}

install_rpm() {
  echo "Instalando paquete .rpm..."

  URL=$(get_asset_url '\.rpm$')

  if [ -z "$URL" ]; then
    echo "No se encontró paquete .rpm"
    exit 1
  fi

  TMP="/tmp/ludus.rpm"

  curl -fL -o "$TMP" "$URL"

  if command -v dnf >/dev/null 2>&1; then
    sudo dnf install -y "$TMP"
  elif command -v yum >/dev/null 2>&1; then
    sudo yum install -y "$TMP"
  elif command -v zypper >/dev/null 2>&1; then
    sudo zypper install -y "$TMP"
  else
    echo "No se encontró gestor RPM"
    rm -f "$TMP"
    exit 1
  fi

  rm -f "$TMP"

  echo "Instalación completada"
}

install_appimage() {
  echo "Instalando AppImage..."

  URL=$(get_asset_url '\.AppImage$')

  if [ -z "$URL" ]; then
    echo "No se encontró AppImage"
    exit 1
  fi

  echo "Descargando:"
  echo "$URL"

  mkdir -p "$INSTALL_DIR"

  curl -fL -o "$INSTALL_DIR/$BINARY_NAME" "$URL"

  chmod +x "$INSTALL_DIR/$BINARY_NAME"

  echo
  echo "Instalado en:"
  echo "$INSTALL_DIR/$BINARY_NAME"
  echo
  echo "Ejecuta:"
  echo "$BINARY_NAME"
}

main() {
  detect_os
  get_latest_version

  echo "Sistema detectado: $OS"

  case "$OS" in
    ubuntu|debian|linuxmint|pop)
      install_deb
      ;;
    fedora|centos|rhel|opensuse*|suse)
      install_rpm
      ;;
    *)
      echo "Usando AppImage..."
      install_appimage
      ;;
  esac
}

main
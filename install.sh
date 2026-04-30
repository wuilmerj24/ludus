#!/bin/sh
set -e

REPO="wuilmerj24/ludus"
BINARY_NAME="ludus"
INSTALL_DIR="$HOME/.local/bin"

detect_arch() {
  ARCH=$(uname -m)
  case "$ARCH" in
    x86_64) ARCH="amd64" ;;
    aarch64) ARCH="arm64" ;;
    *)
      echo "Arquitectura no soportada: $ARCH"
      exit 1
      ;;
  esac
}

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

  FINAL_URL=$(curl -sL -o /dev/null -w '%{url_effective}' \
    "https://github.com/$REPO/releases/latest")

  case "$FINAL_URL" in
    *"/tag/"*)
      LATEST=$(echo "$FINAL_URL" | sed 's#.*/tag/##')
      ;;
    *)
      echo "Error: no hay release marcado como 'Latest'"
      echo "Solución: marca un release como latest en GitHub"
      exit 1
      ;;
  esac

  VERSION_NO_V=$(echo "$LATEST" | sed 's/^v//')

  echo "Última versión: $LATEST"
}

install_appimage() {
  echo "Instalando vía AppImage..."

  mkdir -p "$INSTALL_DIR"

  URL="https://github.com/$REPO/releases/download/$LATEST/${BINARY_NAME}_${VERSION_NO_V}_${ARCH}.AppImage"

  curl -L "$URL" -o "$INSTALL_DIR/$BINARY_NAME"
  chmod +x "$INSTALL_DIR/$BINARY_NAME"

  echo "Instalado en: $INSTALL_DIR/$BINARY_NAME"
  echo "Asegúrate de tener $INSTALL_DIR en tu PATH"
}

install_deb() {
  echo "Instalando vía .deb..."

  URL="https://github.com/$REPO/releases/download/$LATEST/${BINARY_NAME}_${VERSION_NO_V}_${ARCH}.deb"
  TMP="/tmp/ludus.deb"
    # Verificar tamaño mínimo (evita HTML/errores)
  if [ ! -s "$TMP" ] || [ "$(stat -c%s "$TMP")" -lt 10000 ]; then
    echo "Error: descarga inválida (.deb corrupto o inexistente)"
    exit 1
  fi
  curl -L "$URL" -o "$TMP"

  sudo dpkg -i "$TMP" || sudo apt-get install -f -y

  rm -f "$TMP"

  echo "Instalado correctamente"
}

install_rpm() {
  echo "Instalando vía .rpm..."

  URL="https://github.com/$REPO/releases/download/$LATEST/${BINARY_NAME}-${VERSION_NO_V}-1.x86_64.rpm"

  if command -v dnf >/dev/null 2>&1; then
    sudo dnf install -y "$URL"
  elif command -v yum >/dev/null 2>&1; then
    sudo yum install -y "$URL"
  elif command -v zypper >/dev/null 2>&1; then
    sudo zypper install -y "$URL"
  else
    echo "No se encontró gestor de paquetes RPM"
    exit 1
  fi
}

main() {
  detect_arch
  detect_os
  get_latest_version

  echo "Sistema detectado: $OS ($ARCH)"

  case "$OS" in
    ubuntu|debian|linuxmint|pop)
      install_deb
      ;;
    fedora|centos|rhel|opensuse*|suse)
      install_rpm
      ;;
    *)
      echo "Distro no soportada directamente, usando AppImage..."
      install_appimage
      ;;
  esac
}

main
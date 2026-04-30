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

  RESPONSE=$(curl -s https://api.github.com/repos/$REPO/releases)

  # tomar el primer tag_name (el más reciente)
  LATEST=$(echo "$RESPONSE" | grep '"tag_name":' | head -n 1 | sed -E 's/.*"([^"]+)".*/\1/')

  if [ -z "$LATEST" ]; then
    echo "No se pudo obtener la versión"
    exit 1
  fi

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

  echo "Descargando: $URL"

  # Descargar correctamente
  if ! curl -fL --retry 3 --retry-delay 2 -o "$TMP" "$URL"; then
    echo "Error: no se pudo descargar el archivo"
    rm -f "$TMP"
    exit 1
  fi

  # Validar tamaño mínimo (~10KB)
  SIZE=$(stat -c%s "$TMP")
  if [ "$SIZE" -lt 10000 ]; then
    echo "Error: archivo inválido ($SIZE bytes)"
    rm -f "$TMP"
    exit 1
  fi

  # Validar formato real .deb
  if ! dpkg-deb -I "$TMP" >/dev/null 2>&1; then
    echo "Error: archivo no es un .deb válido"
    rm -f "$TMP"
    exit 1
  fi

  echo "Instalando paquete..."
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
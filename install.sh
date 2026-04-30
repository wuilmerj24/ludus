#!/bin/sh
set -e

REPO="wuilmerj24/ludus"
BINARY_NAME="ludus"
INSTALL_DIR="$HOME/.local/bin"

detect_arch() {
  ARCH=$(uname -m)
  case $ARCH in
    x86_64) ARCH="amd64" ;;
    aarch64) ARCH="arm64" ;;
    *) echo "Arquitectura $ARCH no soportada"; exit 1 ;;
  esac
}

detect_os() {
  if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS=$ID
  else
    echo "No se pudo detectar la distro"; exit 1
  fi
}

get_latest_version() {
  LATEST=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
  if [ -z "$LATEST" ]; then
    echo "No se pudo obtener la última versión"
    exit 1
  fi
  VERSION_NO_V=$(echo $LATEST | sed 's/v//')
}

install_appimage() {
  echo "Instalando Ludus $LATEST vía AppImage..."
  mkdir -p "$INSTALL_DIR"
  URL="https://github.com/$REPO/releases/download/$LATEST/${BINARY_NAME}_${VERSION_NO_V}_${ARCH}.AppImage"
  curl -L "$URL" -o "$INSTALL_DIR/$BINARY_NAME"
  chmod +x "$INSTALL_DIR/$BINARY_NAME"
  echo "Ludus instalado en $INSTALL_DIR/$BINARY_NAME"
  echo "Asegúrate de tener $INSTALL_DIR en tu PATH"
}

install_deb() {
  echo "Instalando Ludus $LATEST vía .deb..."
  URL="https://github.com/$REPO/releases/download/$LATEST/${BINARY_NAME}_${VERSION_NO_V}_${ARCH}.deb"
  TMP_DEB="/tmp/ludus.deb"
  curl -L "$URL" -o "$TMP_DEB"
  sudo dpkg -i "$TMP_DEB" || sudo apt-get install -f -y
  rm "$TMP_DEB"
  echo "Ludus instalado. Ejecuta: ludus"
}

install_rpm() {
  echo "Instalando Ludus $LATEST vía .rpm..."
  URL="https://github.com/$REPO/releases/download/$LATEST/${BINARY_NAME}-${VERSION_NO_V}-1.x86_64.rpm"
  if command -v dnf > /dev/null; then
    sudo dnf install -y "$URL"
  elif command -v yum > /dev/null; then
    sudo yum install -y "$URL"
  elif command -v zypper > /dev/null; then
    sudo zypper install -y "$URL"
  else
    echo "No se encontró dnf/yum/zypper"
    exit 1
  fi
  echo "Ludus instalado. Ejecuta: ludus"
}

main() {
  detect_arch
  detect_os
  get_latest_version
  
  echo "Detectado: $OS $ARCH"
  
  case $OS in
    ubuntu|debian|linuxmint|pop)
      install_deb
      ;;
    fedora|centos|rhel|opensuse*)
      install_rpm
      ;;
    *)
      echo "Distro no reconocida. Usando AppImage como fallback..."
      install_appimage
      ;;
  esac
}

main
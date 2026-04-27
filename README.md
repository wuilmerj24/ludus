# 🎮 Ludus

## Ludus es una aplicación open source desarrollada con Rust + Tauri, enfocada en Linux, cuyo objetivo es ofrecer diagnóstico, gestión y optimización de GPUs en un entorno unificado y moderno.
* Pensado para gamers, desarrolladores y usuarios avanzados que quieren control real sobre su sistema gráfico en Linux.

## Objetivo del Proyecto
Ludus busca resolver la fragmentación en Linux respecto a GPUs:
* Detectar y analizar hardware gráfico
* Simplificar la gestión de drivers
* Ofrecer herramientas de monitoreo
* Optimizar el rendimiento en juegos

## Stack Tecnológico
| Capa | Tecnología |
|------|------------|
| Backend | Rust |
| Frontend | Tauri 2.0 |
| Plataforma | Linux (exclusivo) |

## Roadmap

### 🟢 Fase 1 — System & GPU Detection (MVP)
Objetivo: Proveer visibilidad completa del sistema y hardware gráfico.

**Funcionalidades**
* Información del sistema
    * Distro (nombre y versión)
    * Kernel
    * Arquitectura
* Detección de GPU:
    * AMD
    * Intel
    * NVIDIA
    * Configuraciones híbridas (laptops)
* Monitoreo básico de GPU (en evaluación):
    * Uso (utilization)
    * Memoria
    * Temperatura (si está disponible)
* Registro de actividad (experimental):
    * Eventos relacionados con GPU
    * Logs relevantes del sistema

### 🟡 Fase 2 — Driver Management
Objetivo: Automatizar y simplificar la instalación y gestión de drivers.

Funcionalidades
* Instalación de drivers:
    * AMD
    * NVIDIA (propietario)
    * Intel
* Detección automática:
    * Driver actual
    * Driver recomendado
* Actualización de drivers
* Validaciones:
    * Compatibilidad con kernel
    * Compatibilidad con distro

Retos técnicos
* Soporte multi-distro
* Manejo de permisos

### 🔴 Fase 3 — GameMode & Optimización
Objetivo: Mejorar el rendimiento del sistema durante gaming o cargas intensivas.

**Estrategia inicial**

Integración con:
* [Feral GameMode](https://github.com/feralinteractive/gamemode)

Funcionalidades
* Activación automática de optimizaciones:
    * CPU governor (performance)
    * Prioridad de procesos
* Modo gaming:
    * Aplicación de perfiles al lanzar juegos
* Tweaks del sistema:
    * Ajustes temporales durante ejecución

Posible evolución
* Implementación de un GameMode propio en Rust
* Perfiles personalizados por usuario

Instalación (Desarrollo)
```bash
git clone https://github.com/wuilmerj24/ludus
cd ludus
yarn tauri dev
```

## Estado del Proyecto
🚧 En desarrollo activo — Fase 1 (MVP)

Casos de Uso
* Verificar compatibilidad de GPU en Linux
* Diagnosticar problemas de drivers
* Preparar el sistema para gaming
* Monitorear comportamiento de GPU

## Contribuciones
* Se aceptan contribuciones:
* Testing en diferentes distros y Gpus


## Visión

Convertirse en una herramienta estándar en Linux para:
* Gestión de GPUs
* Optimización de rendimiento
* Diagnóstico técnico avanzado



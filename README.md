# 🎮 Ludus

> A modern, native GPU management, monitoring, and optimization tool for Linux, built entirely with Rust.

Ludus is an open-source desktop application designed to provide Linux users with a unified experience for GPU diagnostics, driver management, and gaming performance optimization.

Built with **Rust** and **Iced**, Ludus focuses on performance, safety, and a fully native user interface without relying on web technologies.

---

## ✨ Features

### Current

- Detect AMD, NVIDIA, and Intel GPUs
- Display detailed system information
- Monitor GPU metrics
- Support hybrid GPU configurations
- View GPU-related system logs

### Planned

- Driver installation and updates
- Automatic driver recommendations
- Gaming optimization profiles
- Integration with Linux gaming tools
- Advanced diagnostics
- Performance tuning

---

# 🎯 Project Goals

Linux GPU management is still fragmented across different distributions and command-line tools.

Ludus aims to provide a single application that makes GPU management simple while remaining powerful enough for advanced users.

The long-term vision includes:

- 🔍 Hardware detection
- 📊 Real-time monitoring
- 🔧 Driver management
- 🎮 Gaming optimization
- 🛠 Advanced diagnostics

Whether you're a gamer, developer, or Linux enthusiast, Ludus is designed to give you complete control over your graphics hardware.

---

# 🦀 Tech Stack

| Layer | Technology |
|--------|------------|
| Language | Rust |
| UI | Iced |
| Platform | Linux |

---

# 🗺 Roadmap

## ✅ Phase 1 — System & GPU Detection

**Status:** Completed

### Features

- System information
    - Distribution
    - Kernel version
    - Architecture

- GPU detection
    - AMD
    - NVIDIA
    - Intel
    - Hybrid graphics

- GPU monitoring
    - Utilization
    - Memory usage
    - Temperature (when supported)

- GPU event logs
    - Driver-related events
    - Relevant system logs

---

## 🚧 Phase 2 — Driver Management

**Status:** In Progress

### Planned Features

- Driver installation
    - AMD
    - NVIDIA Proprietary
    - Intel

- Automatic detection
    - Installed driver
    - Recommended driver

- Driver updates

- Compatibility validation
    - Linux distribution
    - Kernel version

### Challenges

- Multi-distribution support
- Privilege management
- Distribution-specific package managers

---

## 🎮 Phase 3 — Gaming Optimization

### Planned Integrations

- Feral GameMode
- MangoHud

### Features

- Automatic performance mode
- CPU governor optimization
- Process priority adjustments
- Gaming profiles
- Temporary system tweaks

### Future Vision

- Native Rust optimization engine
- Custom performance profiles
- Automatic game detection

---

# 🚀 Installation

## Development

```bash
git clone https://github.com/wuilmerj24/ludus.git

cd ludus

cargo run
```

## Install Script

```bash
curl -fsSL https://raw.githubusercontent.com/wuilmerj24/ludus/main/install.sh | sh
```

---

# 📈 Project Status

| Phase | Status |
|--------|--------|
| System Detection | ✅ Complete |
| Driver Management | 🚧 In Progress |
| Gaming Optimization | ⏳ Planned |

---

# 💡 Use Cases

- Prepare Linux systems for gaming
- Monitor GPU performance
- Diagnose graphics-related issues
- Manage GPU drivers
- Optimize gaming performance

---

# 🤝 Contributing

Contributions are always welcome.

You can help by:

- Testing on different Linux distributions
- Testing different GPU vendors
- Reporting bugs
- Improving documentation
- Implementing new features

---

# 🌍 Vision

Ludus aims to become the standard GPU management application for Linux by providing:

- Native performance
- Modern interface
- Reliable diagnostics
- Driver management
- Gaming optimization
- Open-source transparency

---

## 📜 License

This project is open source and licensed under the MIT License.
use std::{collections::HashMap, fs::{read_link, read_to_string, symlink_metadata}, path::{Path, PathBuf}, process::Command};

use all_smi::device::GpuInfo;

#[derive(Debug, Clone)]
pub enum Vendor {
    Nvidia,
    Amd,
    Intel,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct GpuDriverStatus {
    pub name: String,
    pub vendor: Vendor,
    pub driver_installed: bool,
    pub driver_version: Option<String>,
    pub active_kernel_module: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverStatus {
    Installed {
        version: String,
        driver_name: String,
    },
    NotInstalled,
    Unknown(String),
}

#[derive(Debug, Clone)]
pub enum DriverAction {
    Install,
    Update,
    Uninstall,
}

pub struct DriverManager;

impl DriverManager {
    pub fn new() -> Self {
        Self
    }

    pub fn check_driver_status(
        &self,
        gpu_name: &str,
        detail: &HashMap<String, String>,
    ) -> DriverStatus {
        let name_lower = gpu_name.to_lowercase();
        let driver_version_from_detail = detail
            .get("Driver Version")
            .filter(|v| !v.is_empty() && *v != "N/A");

        if let Some(version) = driver_version_from_detail {
            let driver_name = self.guess_driver_name(&name_lower, detail);
            return DriverStatus::Installed {
                version: version.clone(),
                driver_name,
            };
        }

        if name_lower.contains("nvidia") {
            self.check_nvidia_fallback()
        } else if name_lower.contains("amd") || name_lower.contains("radeon") {
            self.check_amd_fallback()
        } else if name_lower.contains("intel") {
            self.check_intel_fallback()
        } else {
            DriverStatus::NotInstalled
        }
    }  

    fn guess_driver_name(&self, gpu_name: &str, detail: &HashMap<String, String>) -> String {
        if gpu_name.contains("nvidia") {
            "NVIDIA Proprietary / NVML".to_string()
        } else if gpu_name.contains("amd") || gpu_name.contains("radeon") {
            // En AMD/Mesa, el campo ASIC Name (ej. "Renoir") nos ayuda a confirmar la familia
            if let Some(asic) = detail.get("ASIC Name") {
                format!("amdgpu / Mesa ({})", asic)
            } else {
                "amdgpu / Mesa".to_string()
            }
        } else if gpu_name.contains("intel") {
            "i915 / Xe (Mesa)".to_string()
        } else {
            "Generic Linux Driver".to_string()
        }
    }

    fn check_nvidia_fallback(&self) -> DriverStatus {
        if Path::new("/proc/driver/nvidia/version").exists() {
            if let Ok(proc_info) = read_to_string("/proc/driver/nvidia/version") {
                let version = proc_info
                    .lines()
                    .next()
                    .unwrap_or("Desconocida")
                    .to_string();

                return DriverStatus::Installed {
                    version,
                    driver_name: "nvidia".into(),
                };
            }
        }

        if Path::new("/sys/module/nouveau").exists() {
            return DriverStatus::Installed {
                version: "Kernel Open-Source".into(),
                driver_name: "nouveau".into(),
            };
        }

        DriverStatus::NotInstalled
    }

    fn check_amd_fallback(&self) -> DriverStatus {
        if Path::new("/sys/module/amdgpu").exists() {
            DriverStatus::Installed {
                version: "In-Kernel".into(),
                driver_name: "amdgpu".into(),
            }
        } else {
            DriverStatus::NotInstalled
        }
    }

    fn check_intel_fallback(&self) -> DriverStatus {
        if Path::new("/sys/module/xe").exists() || Path::new("/sys/module/i915").exists() {
            DriverStatus::Installed {
                version: "In-Kernel".into(),
                driver_name: "intel".into(),
            }
        } else {
            DriverStatus::NotInstalled
        }
    }

    pub fn build_install_command(&self,os_name: &str, vendor: Vendor) -> Result<Command, String> {
        let os_lower = os_name.to_lowercase();
        
        let mut cmd = Command::new("pkexec");
        
        cmd.env("DISPLAY", std::env::var("DISPLAY").unwrap_or(":0".to_string()));
        cmd.env("XAUTHORITY", std::env::var("XAUTHORITY").unwrap_or_default());

        if os_lower.contains("debian") || os_lower.contains("ubuntu") || os_lower.contains("pop") || os_lower.contains("mint") {
            cmd.arg("sh").arg("-c").arg(match vendor {
                Vendor::Nvidia => "apt-get update && apt-get install -y nvidia-driver nvidia-vulkan-ic libva-nvidia-driver vulkan-tools",
                Vendor::Amd => "apt-get update && apt-get install -y firmware-amd-graphics libdrm-amdgpu1 libgl1-mesa-dri mesa-va-drivers mesa-vdpau-drivers mesa-vulkan-drivers xserver-xorg-video-amdgpu vainfo vulkan-tools",
                Vendor::Intel => "apt-get update && apt-get install -y intel-media-va-driver-non-free mesa-va-drivers mesa-vulkan-drivers vainfo vulkan-tools",
                Vendor::Unknown => return Err("GPU no reconocida".into()),
            });
        } else if os_lower.contains("arch") || os_lower.contains("cachy") || os_lower.contains("endeavour") || os_lower.contains("manjaro") {
            cmd.arg("sh").arg("-c").arg(match vendor {
                Vendor::Nvidia => "pacman -Syu --noconfirm --needed nvidia-dkms nvidia-utils lib32-nvidia-utils libva-nvidia-driver vulkan-tools",
                Vendor::Amd => "pacman -Syu --noconfirm --needed mesa lib32-mesa vulkan-radeon lib32-vulkan-radeon libva-mesa-driver lib32-libva-mesa-driver xf86-video-amdgpu vulkan-tools",
                Vendor::Intel => "pacman -Syu --noconfirm --needed mesa lib32-mesa vulkan-intel lib32-vulkan-intel intel-media-driver libva-intel-driver vulkan-tools",
                Vendor::Unknown => return Err("GPU no reconocida".into()),
            });
        } else if os_lower.contains("fedora") || os_lower.contains("nobara") {
            cmd.arg("sh").arg("-c").arg(match vendor {
                Vendor::Nvidia => "dnf install -y https://mirrors.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm https://mirrors.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm && dnf install -y akmod-nvidia xorg-x11-drv-nvidia-cuda libva-nvidia-driver && echo 'options nvidia-drm modeset=1 fbdev=1' > /etc/modprobe.d/nvidia-wayland.conf",
                Vendor::Amd => "dnf install -y mesa-dri-drivers mesa-vulkan-drivers mesa-va-drivers xorg-x11-drv-amdgpu libva-utils vulkan-tools",
                Vendor::Intel => "dnf install -y mesa-dri-drivers mesa-vulkan-drivers intel-media-driver libva-utils vulkan-tools",
                Vendor::Unknown => return Err("GPU no reconocida".into()),
            });
        } else if os_lower.contains("suse") {
            cmd.arg("sh").arg("-c").arg(match vendor {
                Vendor::Nvidia => "zypper addrepo --refresh https://download.nvidia.com/opensuse/tumbleweed NVIDIA || true; zypper install -y nvidia-video-G06 nvidia-gl-G06 libva-nvidia-driver",
                Vendor::Amd => "zypper install -y Mesa Mesa-dri libvulkan_radeon Mesa-libva xf86-video-amdgpu",
                Vendor::Intel => "zypper install -y Mesa libvulkan_intel intel-media-driver",
                Vendor::Unknown => return Err("GPU no reconocida".into()),
            });
        } else {
            return Err(format!("SO '{}' no compatible", os_name));
        }

        Ok(cmd)
    }
}
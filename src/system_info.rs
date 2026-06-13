use std::fs;

use serde::{Deserialize, Serialize};
use sysinfo::{Disks, System};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_name: String,
    pub kernel_version: String,
    pub cpu: String,
    pub temp_cpu: Option<f32>,
    pub total_ram: String,
    pub total_storage: String,
    pub is_monitor: bool,
}

impl SystemInfo {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let os_name = System::long_os_version().unwrap_or_default();

        let name = os_name
            .split('(')
            .nth(1)
            .unwrap_or(&os_name)
            .split(')')
            .next()
            .unwrap_or(&os_name)
            .replace(" GNU/Linux", "")
            .split_whitespace()
            .next()
            .unwrap_or("Unknown")
            .to_string();

        let version = System::os_version().unwrap_or_default();

        let kernel_version =
            System::kernel_version().unwrap_or_default();

        let cpu_name = sys
            .cpus()
            .first()
            .map(|cpu| cpu.brand())
            .unwrap_or("Unknown");

        let cpu_name = cpu_name
            .split(" with ")
            .next()
            .unwrap_or(cpu_name)
            .split(" w/")
            .next()
            .unwrap_or(cpu_name)
            .replace("(R)", "")
            .replace("(TM)", "")
            .trim()
            .to_string();

        let total_ram_bytes = sys.total_memory();

        let total_ram = format!(
            "{} GB",
            (total_ram_bytes as f64
                / 1024.0
                / 1024.0
                / 1024.0)
                .round() as u64
        );

        let disks = Disks::new_with_refreshed_list();

        let total_storage_bytes: u64 = disks
            .iter()
            .map(|disk| disk.total_space())
            .sum();

        let total_storage =
            Self::format_storage(total_storage_bytes);

        Self {
            os_name: format!("{} {}", name, version),
            kernel_version,
            cpu: cpu_name,
            temp_cpu: Self::cpu_temp(),
            total_ram,
            total_storage,
            is_monitor: false,
        }
    }

    fn format_storage(bytes: u64) -> String {
        const KB: f64 = 1024.0;
        const MB: f64 = KB * 1024.0;
        const GB: f64 = MB * 1024.0;
        const TB: f64 = GB * 1024.0;

        let bytes = bytes as f64;

        if bytes >= TB {
            format!("{:.1} TB", bytes / TB)
        } else if bytes >= GB {
            format!("{:.0} GB", bytes / GB)
        } else if bytes >= MB {
            format!("{:.0} MB", bytes / MB)
        } else {
            format!("{:.0} B", bytes)
        }
    }

    fn cpu_temp() -> Option<f32> {
        #[cfg(target_os = "linux")]
        {
            Self::cpu_temp_linux()
        }

        #[cfg(target_os = "windows")]
        {
            None
        }

        #[cfg(target_os = "macos")]
        {
            None
        }
    }

    pub fn cpu_temp_linux() -> Option<f32> {
        let hwmons = fs::read_dir("/sys/class/hwmon").ok()?;

        for hwmon in hwmons.flatten() {
            let path = hwmon.path();

            let name = fs::read_to_string(path.join("name"))
                .unwrap_or_default();

            if name.contains("k10temp")
                || name.contains("coretemp")
                || name.contains("zenpower")
            {
                let temp = fs::read_to_string(
                    path.join("temp1_input")
                )
                .ok()?;

                let value =
                    temp.trim().parse::<f32>().ok()?;

                return Some(value / 1000.0);
            }
        }

        None
    }
}
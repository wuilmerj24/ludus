use std::fs;

use all_smi::AllSmi;
use serde::{Deserialize, Serialize};
use sysinfo::{
    Components, Disks, Networks, System,
};
use tauri::ipc::IpcResponse;

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct SoInfo{
    pub name:String,
    pub kernel_version:String,
    pub os_version:String,
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct HardwareInfo{
    pub name:String,
    pub pci_ex_info:String,
    pub ram:String,
    pub version_driver:String,
    pub date:String,    
}

#[tauri::command]
fn get_so_info()->SoInfo{
    let mut sys = System::new_all();
    sys.refresh_all();
    
    SoInfo { name: System::name().unwrap().to_string(), kernel_version: System::kernel_version().unwrap().to_string(), os_version: System::os_version().unwrap().to_string() }
    
}

#[tauri::command]
fn get_hardware_info() -> Vec<HardwareInfo> {
    let mut gpus: Vec<HardwareInfo> = Vec::new();

    let smi = AllSmi::new().unwrap();

    for gpu in smi.get_gpu_info() {
        // 🔑 ESTE ES EL DATO CLAVE QUE YA TIENES
        let bus_id = gpu.detail.get("PCI Bus").unwrap();

        // 🔥 BUS → cardX
        let card = get_card_from_bus(&bus_id);

        // 🎯 PCIe
        let pcie = card
            .as_ref()
            .and_then(|c| get_pcie_info(c));

        // 🎯 VRAM
        let mut vram_gb: Option<u64> = None;

        if let Some(ref card_name) = card {
            // AMD
            vram_gb = get_amd_vram(card_name);
        }

        // NVIDIA fallback
        if vram_gb.is_none() {
            vram_gb = Some((gpu.total_memory / 1024) as u64);
        }

        let formatted = format_gpu_info(pcie, vram_gb);

        let data = HardwareInfo {
            name: gpu.name,
            pci_ex_info: formatted,
            ram: format!("{} MB", gpu.total_memory),
            version_driver: gpu
                .detail
                .get("Driver Version")
                .unwrap_or(&"Unknown".into())
                .to_string(),
            date: String::from("Unknown"),
        };

        gpus.push(data);
    }
    
    gpus
}

fn read(path: &str) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

// 🔑 BUS → cardX
fn get_card_from_bus(bus_id: &str) -> Option<String> {
    let path = format!("/sys/bus/pci/devices/{}/drm", bus_id);

    let entries = fs::read_dir(path).ok()?;

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();

        if name.starts_with("card") {
            return Some(name);
        }
    }

    None
}

// 🔥 PCIe real
fn get_pcie_info(card: &str) -> Option<String> {
    let base = format!("/sys/class/drm/{}/device", card);

    let speed = read(&(base.clone() + "/max_link_speed"))
        .or_else(|| read(&(base.clone() + "/current_link_speed")))?;

    let width = read(&(base.clone() + "/max_link_width"))
        .or_else(|| read(&(base.clone() + "/current_link_width")))?;

    let gen = if speed.contains("2.5") {
        "PCIe 1.0"
    } else if speed.contains("5.0") {
        "PCIe 2.0"
    } else if speed.contains("8.0") {
        "PCIe 3.0"
    } else if speed.contains("16.0") {
        "PCIe 4.0"
    } else if speed.contains("32.0") {
        "PCIe 5.0"
    } else {
        "PCIe ?"
    };

    Some(format!("{} x{}", gen, width))
}

// AMD VRAM
fn get_amd_vram(card: &str) -> Option<u64> {
    let path = format!("/sys/class/drm/{}/device/mem_info_vram_total", card);
    let bytes: u64 = fs::read_to_string(path).ok()?.trim().parse().ok()?;
    Some(bytes / 1024 / 1024 / 1024)
}

fn format_gpu_info(pcie: Option<String>, vram_gb: Option<u64>) -> String {
    let pcie_part = pcie.unwrap_or("Unknown PCIe".into());

    let memory_part = match vram_gb {
        Some(v) => format!("{}GB", v),
        None => "Shared".into(),
    };

    format!("{} | {}", pcie_part, memory_part)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_so_info,
            get_hardware_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

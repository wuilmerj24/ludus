use std::panic::{AssertUnwindSafe, catch_unwind};

use all_smi::{AllSmi, device::{CpuInfo, GpuInfo}};

#[derive(Debug)]
pub enum GpuError {
    BackendError(String),
    NoGpusDetected,
}

#[derive()]
pub struct EGpuInfo{
    all_smi:AllSmi,
}
#[derive()]
pub struct EGpuInfoData{
    
}

impl EGpuInfo {
    pub fn new()->Result<Self,all_smi::Error> {
        let smi = AllSmi::new()?;
        Ok(
            Self {
                all_smi:smi,
            }
        )
    }
    
    pub fn get_gpu_by_uuid(&mut self)->Option<GpuInfo>{
        let uuid = self.all_smi
            .get_gpu_info()
            .iter()
            .find(|gpu| gpu.used_memory > 0)
            .map(|gpu| gpu.uuid.clone())
            .unwrap_or_default();
        self.all_smi.get_gpu_by_uuid(&uuid)
    }
    
    pub fn get_all_gpu(&mut self,is_test:bool) -> Vec<GpuInfo> {
        let result = catch_unwind(AssertUnwindSafe(|| {
            let gpus = self.all_smi.get_gpu_info();
            println!("gpus {:?}",gpus.get(0).unwrap().name);
            gpus
        }));
    
        match result {
            Ok(gpus) =>{
                if is_test {
                    Self::generate_example_gpus()
                }else{
                    gpus
                }
            },
            Err(_) => Vec::new(), // fallback seguro
        }
    }
    
    pub fn get_all_cpu(&mut self) -> Vec<CpuInfo> {
        let result = catch_unwind(AssertUnwindSafe(|| {
            let cpus = self.all_smi.get_cpu_info();
            println!("gpus {:?}",cpus.get(0).unwrap().temperature);
            cpus
        }));
    
        match result {
            Ok(cpus) => cpus,
            Err(_) => Vec::new(), // fallback seguro
        }
    }
    
    fn generate_example_gpus() -> Vec<GpuInfo> {
        let mut gpus = Vec::new();
        
        let mut detail1 = std::collections::HashMap::new();
        detail1.insert("VBIOS Date".to_string(), "2021/02/28 08:58".to_string());
        detail1.insert("VBIOS Version".to_string(), "017.010.000.029.000000".to_string());
        detail1.insert("Driver Version".to_string(), "3.64.0".to_string());
        detail1.insert("PCI Bus".to_string(), "0000:0b:00.0".to_string());
        detail1.insert("Device ID".to_string(), "0x1638".to_string());
        detail1.insert("ASIC Name".to_string(), "Renoir".to_string());
        detail1.insert("Memory Clock".to_string(), "1600 MHz".to_string());
        detail1.insert("Revision ID".to_string(), "0xc8".to_string());
        detail1.insert("Device Name".to_string(), "NVIDIA GeForce RTX 4090".to_string());
        
        let gpu1 = GpuInfo {
            uuid: "GPU-0000:0b:00.0".to_string(),
            time: "2026-06-30 14:40:43".to_string(),
            name: "NVIDIA GeForce RTX 4090".to_string(),
            device_type: "GPU".to_string(),
            host_id: "wuilmerj24-pc".to_string(),
            hostname: "wuilmerj24-pc".to_string(),
            instance: "wuilmerj24-pc".to_string(),
            utilization: 85.0,
            ane_utilization: 0.0,
            dla_utilization: None,
            tensorcore_utilization: None,
            temperature: 72,
            used_memory: 19327352832, // 18 GB
            total_memory: 25769803776, // 24 GB
            frequency: 1850,
            power_consumption: 320.5,
            gpu_core_count: None,
            temperature_threshold_slowdown: None,
            temperature_threshold_shutdown: None,
            temperature_threshold_max_operating: None,
            temperature_threshold_acoustic: None,
            performance_state: None,
            numa_node_id: None,
            gsp_firmware_mode: None,
            gsp_firmware_version: None,
            nvlink_remote_devices: Vec::new(),
            gpm_metrics: None,
            detail: detail1,
        };
        gpus.push(gpu1);
        
        let mut detail2 = std::collections::HashMap::new();
        detail2.insert("VBIOS Date".to_string(), "2022/05/15 10:30".to_string());
        detail2.insert("VBIOS Version".to_string(), "018.020.001.035.000000".to_string());
        detail2.insert("Driver Version".to_string(), "4.12.0".to_string());
        detail2.insert("PCI Bus".to_string(), "0000:0c:00.0".to_string());
        detail2.insert("Device ID".to_string(), "0x2234".to_string());
        detail2.insert("ASIC Name".to_string(), "GA102".to_string());
        detail2.insert("Memory Clock".to_string(), "1800 MHz".to_string());
        detail2.insert("Revision ID".to_string(), "0xa2".to_string());
        detail2.insert("Device Name".to_string(), "NVIDIA GeForce RTX 3080".to_string());
        
        let gpu2 = GpuInfo {
            uuid: "GPU-0000:0c:00.0".to_string(),
            time: "2026-06-30 14:40:43".to_string(),
            name: "NVIDIA GeForce RTX 3080".to_string(),
            device_type: "GPU".to_string(),
            host_id: "wuilmerj24-pc".to_string(),
            hostname: "wuilmerj24-pc".to_string(),
            instance: "wuilmerj24-pc".to_string(),
            utilization: 0.0,
            ane_utilization: 0.0,
            dla_utilization: None,
            tensorcore_utilization: None,
            temperature: 35,
            used_memory: 0,
            total_memory: 17179869184, // 16 GB
            frequency: 400,
            power_consumption: 0.0,
            gpu_core_count: None,
            temperature_threshold_slowdown: None,
            temperature_threshold_shutdown: None,
            temperature_threshold_max_operating: None,
            temperature_threshold_acoustic: None,
            performance_state: None,
            numa_node_id: None,
            gsp_firmware_mode: None,
            gsp_firmware_version: None,
            nvlink_remote_devices: Vec::new(),
            gpm_metrics: None,
            detail: detail2,
        };
        gpus.push(gpu2);
        
        let mut detail3 = std::collections::HashMap::new();
        detail3.insert("VBIOS Date".to_string(), "2023/01/20 12:00".to_string());
        detail3.insert("VBIOS Version".to_string(), "019.030.002.042.000000".to_string());
        detail3.insert("Driver Version".to_string(), "5.1.0".to_string());
        detail3.insert("PCI Bus".to_string(), "0000:0d:00.0".to_string());
        detail3.insert("Device ID".to_string(), "0x3366".to_string());
        detail3.insert("ASIC Name".to_string(), "Navi 23".to_string());
        detail3.insert("Memory Clock".to_string(), "2000 MHz".to_string());
        detail3.insert("Revision ID".to_string(), "0x04".to_string());
        detail3.insert("Device Name".to_string(), "AMD Radeon RX 6700 XT".to_string());
        
        let gpu3 = GpuInfo {
            uuid: "GPU-0000:0d:00.0".to_string(),
            time: "2026-06-30 14:40:43".to_string(),
            name: "AMD Radeon RX 6700 XT".to_string(),
            device_type: "GPU".to_string(),
            host_id: "wuilmerj24-pc".to_string(),
            hostname: "wuilmerj24-pc".to_string(),
            instance: "wuilmerj24-pc".to_string(),
            utilization: 0.0,
            ane_utilization: 0.0,
            dla_utilization: None,
            tensorcore_utilization: None,
            temperature: 32,
            used_memory: 0,
            total_memory: 12884901888, // 12 GB
            frequency: 300,
            power_consumption: 0.0,
            gpu_core_count: None,
            temperature_threshold_slowdown: None,
            temperature_threshold_shutdown: None,
            temperature_threshold_max_operating: None,
            temperature_threshold_acoustic: None,
            performance_state: None,
            numa_node_id: None,
            gsp_firmware_mode: None,
            gsp_firmware_version: None,
            nvlink_remote_devices: Vec::new(),
            gpm_metrics: None,
            detail: detail3,
        };
        gpus.push(gpu3);
        
        gpus
    }
}
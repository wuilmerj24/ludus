use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Debug)]
pub struct ESystemInfo {
    system: System,
}

impl ESystemInfo {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
        }
    }

    //------------------------
    // Refresh
    //------------------------

    pub fn refresh_all(&mut self) {
        self.system.refresh_all();
    }

    pub fn refresh_cpu(&mut self) {
        self.system.refresh_cpu_all();
    }

    pub fn refresh_memory(&mut self) {
        self.system.refresh_memory();
    }

    //------------------------
    // Sistema
    //------------------------

    pub fn os_name(&self) -> String {
        System::name().unwrap_or_default()
    }

    pub fn os_version(&self) -> String {
        System::os_version().unwrap_or_default()
    }

    pub fn kernel_version(&self) -> String {
        System::kernel_version().unwrap_or_default()
    }

    pub fn hostname(&self) -> String {
        System::host_name().unwrap_or_default()
    }

    //------------------------
    // CPU
    //------------------------

    pub fn cpu_name(&self) -> String {
        self.system.cpus()[0].brand().to_string()
    }

    pub fn cpu_vendor(&self) -> String {
        self.system.cpus()[0].vendor_id().to_string()
    }

    pub fn cpu_frequency(&self) -> u64 {
        self.system.cpus()[0].frequency()
    }

    pub fn cpu_usage(&self) -> f32 {
        self.system.cpus()[0].cpu_usage()
    }

    pub fn logical_cores(&self) -> usize {
        self.system.cpus().len()
    }

    pub fn physical_cores(&self) -> Option<usize> {
        System::physical_core_count()
    }

    //------------------------
    // Memoria
    //------------------------

    pub fn total_memory(&self) -> u64 {
        self.system.total_memory()
    }

    pub fn used_memory(&self) -> u64 {
        self.system.used_memory()
    }

    pub fn available_memory(&self) -> u64 {
        self.system.available_memory()
    }

    pub fn total_swap(&self) -> u64 {
        self.system.total_swap()
    }

    pub fn used_swap(&self) -> u64 {
        self.system.used_swap()
    }
}
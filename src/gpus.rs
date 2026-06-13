use all_smi::{AllSmi, Error, device::GpuInfo};
use serde::{Deserialize, Serialize};

pub struct GpusService {
    pub smi:AllSmi
}

impl GpusService {
    pub async fn new()->Result<Self,Error>{
        let smi = AllSmi::new()?;
        Ok(Self{
            smi:smi
        })
    }
    
    pub fn get_gpus(&self)-> Vec<GpuInfo>{
        self.smi.get_gpu_info()
    }
}
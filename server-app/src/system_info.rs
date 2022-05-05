//! Retrives infomation from the local system

use sysinfo::{ System, SystemExt, ProcessorExt };
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessorInfo {
    pub name: String,
    pub usage: f32,
    pub frequency: u64,
    pub vendor_id: String,
    pub brand: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub hostname: String,
    pub global_processor_info: ProcessorInfo,
    pub processors: Vec<ProcessorInfo>,

    pub total_memory: u64,
    pub free_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub free_swap: u64,
    pub used_swap: u64,
}

pub fn get_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut processors = Vec::new();

    for processor in sys.processors() {
        processors.push(ProcessorInfo {
            name: processor.name().to_owned(),
            usage: processor.cpu_usage(),
            frequency: processor.frequency(),
            vendor_id: processor.vendor_id().to_owned(),
            brand: processor.brand().trim().to_owned(),
        });
    }

    let global_processor_info = ProcessorInfo {
        name: sys.global_processor_info().name().to_owned(),
        usage: sys.global_processor_info().cpu_usage(),
        frequency: sys.global_processor_info().frequency(),
        vendor_id: sys.global_processor_info().vendor_id().to_owned(),
        brand: sys.global_processor_info().brand().trim().to_owned(),
    };

    let system_info = SystemInfo {
        os: std::env::consts::OS.to_owned(),
        arch: std::env::consts::ARCH.to_owned(),
        hostname: sys.host_name().unwrap_or("".to_owned()),

        global_processor_info,
        processors,

        total_memory: sys.total_memory(),
        free_memory: sys.free_memory(),
        used_memory: sys.used_memory(),
        total_swap: sys.total_swap(),
        free_swap: sys.free_swap(),
        used_swap: sys.used_swap(),
    };

    system_info
}
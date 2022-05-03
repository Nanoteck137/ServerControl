use sysinfo::{ System, SystemExt, ProcessorExt };
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
struct ProcessorInfo {
    name: String,
    usage: f32,
    frequency: u64,
    vendor_id: String,
    brand: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SystemInfo {
    os: String,
    arch: String,
    hostname: String,
    global_processor_info: ProcessorInfo,
    processors: Vec<ProcessorInfo>,

    total_memory: u64,
    free_memory: u64,
    used_memory: u64,
    total_swap: u64,
    free_swap: u64,
    used_swap: u64,
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("---- SYSTEM ----");
    println!("System Name: {:?}", sys.name());
    println!("System kernel version: {:?}", sys.kernel_version());
    println!("System os version: {:?}", sys.os_version());
    println!("System os version: {:?}", sys.long_os_version());
    println!("System host name: {:?}", sys.host_name());

    println!("{:#?}", sys);
    println!("{}", std::env::consts::OS);
    println!("{}", std::env::consts::ARCH);

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

    let s = serde_json::to_string(&system_info).unwrap();
    println!("JSON: {}", s);

    println!("Hello, world!");

    let client = reqwest::blocking::Client::new();

    let res = client.post("http://localhost:3000/parse/classes/clients")
        .header("X-Parse-Application-Id", "servercontrol")
        .header("Content-Type", "application/json")
        .body(s)
        .send()
        .unwrap();

    println!("Res: {:?}", res.text());
}

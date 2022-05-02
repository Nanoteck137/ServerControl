use sysinfo::{ System, SystemExt };
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
struct SystemInfo {
    os: String,
    arch: String,
    hostname: String,
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

    let system_info = SystemInfo {
        os: std::env::consts::OS.to_owned(),
        arch: std::env::consts::ARCH.to_owned(),
        hostname: sys.host_name().unwrap_or("".to_owned()),
    };

    let s = serde_json::to_string(&system_info).unwrap();
    println!("JSON: {}", s);

    println!("Hello, world!");
}

use voicevox_core::VVCore;

use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let base_path = if args.len() > 1 {
        PathBuf::from(args[1].clone())
    } else {
        panic!("parent directory of the voicevox core library is needed as command argument.");
    };
    let mut core_path = base_path.clone();
    if cfg!(target_os = "windows") {
        core_path.push("core_cpu_x64.dll");
    } else if cfg!(target_os = "macos") {
        core_path.push("libcore_cpu_universal2.dylib");
    } else {
        core_path.push("libcore_cpu_x64.so");
    }
    let core = VVCore::new(&core_path).unwrap();
    core.initialize(&base_path, false, 0);
    println!("metas:\n");
    println!("{}", core.metas());
    println!();
    println!("supported_devices:\n");
    println!("{}", core.supported_devices());
    core.finalize();
}

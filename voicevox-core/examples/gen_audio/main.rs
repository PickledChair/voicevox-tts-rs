use voicevox_core::VVCore;

use hound;
use npyz::NpyFile;

use std::env;
use std::fs::File;
use std::io;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let data_dir = {
        crate_dir.push("examples");
        crate_dir.push("gen_audio");
        crate_dir.push("data");
        crate_dir
    };

    let mut phoneme_size = 0;
    let mut phoneme = None;
    let mut f0 = None;
    for entry in data_dir.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            if entry.path().is_dir()
                || entry.path().extension().map_or(None, |e| e.to_str()) != Some("npy")
            {
                continue;
            }
            let file = io::BufReader::new(File::open(entry.path())?);
            let npy = NpyFile::new(file)?;

            if entry.path().file_name().unwrap().to_str().unwrap() == "phoneme.npy" {
                phoneme_size = npy.shape()[1] as usize;
                phoneme = Some(npy.into_vec()?);
                continue;
            }
            if entry.path().file_name().unwrap().to_str().unwrap() == "f0.npy" {
                f0 = Some(npy.into_vec()?);
            }
        }
    }

    if phoneme.is_none() {
        panic!("phoneme.npy not found.");
    }
    if f0.is_none() {
        panic!("f0.npy not found.");
    }

    let core = VVCore::new(&core_path).unwrap();
    core.initialize(&base_path, false, 0);
    let wave = core.decode_forward(
        phoneme_size,
        f0.unwrap().as_mut_slice(),
        phoneme.unwrap().as_mut_slice(),
        1,
    );
    core.finalize();

    if wave.is_none() {
        panic!("decode_forward failed.");
    }

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 24000,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create("audio.wav", spec).unwrap();
    for value in wave.unwrap() {
        writer.write_sample(value).unwrap();
    }

    Ok(())
}

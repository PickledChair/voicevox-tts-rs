use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use voicevox_tts::{OpenJTalk, VVCore, VVTTSEngine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(
        args.len(),
        4,
        "require arguments: <core_file_path>, <openjtalk_dict_dir>, text"
    );
    println!("arg1: core_file_path: {}", &args[1]);
    println!("arg2: openjtalk_dict_dir: {}", &args[2]);
    println!("arg3: text: {}", &args[3]);

    let core_file_path = PathBuf::from(&args[1]);
    let root_dir_path = if let Some(path) = core_file_path.parent() {
        path.to_owned()
    } else {
        std::env::current_dir().unwrap()
    };
    println!("root_dir_path: {}", root_dir_path.display());

    // コアライブラリをロード
    println!("loading core library...");
    let core = VVCore::new(&core_file_path)?;
    println!("loaded!");

    // コアライブラリの初期化
    println!("initializing core library...");
    if !core.initialize(&root_dir_path, false, 0) {
        eprintln!(
            "failed to initialize core library: {}",
            &core.last_error_message()
        );
        std::process::exit(1);
    }
    println!("initialized!");

    // OpenJTalk の初期化
    println!("initializing openjtalk...");
    let openjtalk_dict_path = PathBuf::from(&args[2]);
    let openjtalk = OpenJTalk::new(&openjtalk_dict_path)?;
    println!("initialized openjtalk!");

    // VOICEVOX TTS エンジンで音声合成・音声ファイルの書き出し
    let engine = VVTTSEngine::new(openjtalk, core);
    let wav = engine.tts(&args[3], 1)?;
    let mut file = File::create("test.wav")?;
    file.write_all(wav.as_slice())?;
    println!("{}", wav.len());

    // エンジンの終了処理
    engine.finalize();
    Ok(())
}

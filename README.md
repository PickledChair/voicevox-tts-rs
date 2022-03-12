# VOICEVOX TTS Library (written in Rust)

必要な開発環境：

- C/C++ 開発環境
  - gcc または clang
  - CMake（バージョン 3.16 以降）
- Rust toolchain (edition 2021)



## サンプルの実行

TTS のサンプルの実行には [VOICEVOX CORE](https://github.com/VOICEVOX/voicevox_core) と [ONNX Runtime](https://github.com/microsoft/onnxruntime), [OpenJTalk の辞書](https://github.com/r9y9/open_jtalk/releases) が必要です。

以下のコマンドにより音声ファイルが生成されます（macOS の場合、`LD_LIBRARY_PATH` を `DYLD_LIBRARY_PATH` に置き換えてください）。

```
$ LD_LIBRARY_PATH="$LD_LIBRARY_PATH:/path/to/onnxruntime/lib/" \
  cargo run --example simple_tts -- \
  <path to voicevox core library> \
  <root dir of open_jtalk dictionary> \
  <text>
```


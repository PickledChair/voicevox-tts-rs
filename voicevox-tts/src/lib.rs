pub mod acoustic_feature_extractor;
pub mod full_context_label;
pub mod model;
pub mod mora_list;

use std::path::Path;

use model::AudioQueryModel;
pub use openjtalk::OpenJTalk;
use synthesis_engine::SynthesisEngine;
pub use voicevox_core::VVCore;

pub mod synthesis_engine;

pub struct VVTTSEngine {
    openjtalk: OpenJTalk,
    synthesis_engine: SynthesisEngine,
}

impl VVTTSEngine {
    pub fn new(openjtalk: OpenJTalk, core: VVCore) -> VVTTSEngine {
        let synthesis_engine = SynthesisEngine::new(openjtalk, core);
        VVTTSEngine {
            openjtalk,
            synthesis_engine,
        }
    }

    pub fn initialize_openjtalk(&self, openjtalk_dict_path: &Path) -> Result<(), String> {
        self.openjtalk.load(openjtalk_dict_path)
    }

    pub fn tts<T: AsRef<str>>(&self, text: T, speaker_id: i64) -> Result<Vec<u8>, String> {
        let accent_phrases = self
            .synthesis_engine
            .create_accent_phrases(text.as_ref().to_string(), speaker_id)?;
        let audio_query = AudioQueryModel {
            accent_phrases,
            speed_scale: 1.0,
            pitch_scale: 0.0,
            intonation_scale: 1.0,
            volume_scale: 1.0,
            pre_phoneme_length: 0.1,
            post_phoneme_length: 0.1,
            output_sampling_rate: synthesis_engine::DEFAULT_SAMPLING_RATE,
            output_stereo: false,
            kana: "".to_string(),
        };

        self.synthesis_engine
            .synthesis_wave_format(audio_query, speaker_id, true)
    }

    pub fn finalize(&self) {
        self.openjtalk.delete();
        self.synthesis_engine.finalize();
    }
}

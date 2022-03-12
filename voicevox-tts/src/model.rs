#[derive(Debug, Clone)]
pub struct MoraModel {
    pub text: String,
    pub consonant: Option<String>,
    pub consonant_length: Option<f32>,
    pub vowel: String,
    pub vowel_length: f32,
    pub pitch: f32,
}

#[derive(Debug, Clone)]
pub struct AccentPhraseModel {
    pub moras: Vec<MoraModel>,
    pub accent: u32,
    pub pause_mora: Option<MoraModel>,
    pub is_interrogative: bool,
}

#[derive(Debug, Clone)]
pub struct AudioQueryModel {
    pub accent_phrases: Vec<AccentPhraseModel>,
    pub speed_scale: f32,
    pub pitch_scale: f32,
    pub intonation_scale: f32,
    pub volume_scale: f32,
    pub pre_phoneme_length: f32,
    pub post_phoneme_length: f32,
    pub output_sampling_rate: u32,
    pub output_stereo: bool,
    pub kana: String,
}

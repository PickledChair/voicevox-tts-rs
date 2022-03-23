use crate::{
    acoustic_feature_extractor::OjtPhoneme,
    full_context_label::extract_fullcontext,
    model::{AccentPhraseModel, AudioQueryModel, MoraModel},
    mora_list::mora2text,
};
use openjtalk::OpenJTalk;
use voicevox_core::VVCore;

use std::io::{Cursor, Seek, SeekFrom, Write};

pub const UNVOICED_MORA_LIST: &[&str] = &["A", "I", "U", "E", "O", "cl", "pau"];

pub const MORA_PHONEME_LIST: &[&str] = &[
    "a", "i", "u", "e", "o", "N", "A", "I", "U", "E", "O", "cl", "pau",
];

pub fn to_flatten_mora(accent_phrases: &Vec<AccentPhraseModel>) -> Vec<MoraModel> {
    let mut flatten_moras = Vec::new();

    for accent_phrase in accent_phrases.iter() {
        let moras = &accent_phrase.moras;
        for mora in moras.iter() {
            flatten_moras.push(mora.clone());
        }
        if let Some(ref pause_mora) = accent_phrase.pause_mora {
            flatten_moras.push(pause_mora.clone());
        }
    }

    flatten_moras
}

pub fn to_phoneme_data_list(str_list: Vec<String>) -> Vec<OjtPhoneme> {
    OjtPhoneme::convert(
        str_list
            .into_iter()
            .enumerate()
            .map(|(i, string)| OjtPhoneme::new(string, i as f32, (i as f32) + 1.0))
            .collect(),
    )
}

pub fn split_mora(
    phoneme_list: &Vec<OjtPhoneme>,
    consonant_phoneme_list: &mut Vec<OjtPhoneme>,
    vowel_phoneme_list: &mut Vec<OjtPhoneme>,
    vowel_indexes: &mut Vec<i64>,
) {
    for i in 0..phoneme_list.len() {
        let result = MORA_PHONEME_LIST
            .iter()
            .find(|phoneme| *phoneme == &(phoneme_list[i].phoneme));
        if result.is_some() {
            vowel_indexes.push(i as i64);
        }
    }
    for index in vowel_indexes.iter() {
        vowel_phoneme_list.push(phoneme_list[*index as usize].clone());
    }
    consonant_phoneme_list.push(OjtPhoneme::default());
    for i in 0..(vowel_indexes.len() - 1) {
        let prev = vowel_indexes[i];
        let next = vowel_indexes[i + 1];
        if next - prev == 1 {
            consonant_phoneme_list.push(OjtPhoneme::default());
        } else {
            consonant_phoneme_list.push(phoneme_list[next as usize - 1].clone());
        }
    }
}

pub fn adjust_interrogative_accent_phrases(
    accent_phrases: &Vec<AccentPhraseModel>,
) -> Vec<AccentPhraseModel> {
    accent_phrases
        .iter()
        .map(|accent_phrase| AccentPhraseModel {
            moras: adjust_interrogative_moras(accent_phrase.clone()),
            accent: accent_phrase.accent,
            pause_mora: accent_phrase.pause_mora.clone(),
            is_interrogative: accent_phrase.is_interrogative,
        })
        .collect()
}

fn adjust_interrogative_moras(accent_phrase: AccentPhraseModel) -> Vec<MoraModel> {
    let moras = accent_phrase.moras;
    if accent_phrase.is_interrogative && !moras.is_empty() {
        let last_mora = moras.last().unwrap().clone();
        let last_mora_pitch = last_mora.pitch;
        if last_mora_pitch != 0.0 {
            let mut new_moras = moras.clone();
            let interrogative_mora = make_interrogative_mora(last_mora);
            new_moras.push(interrogative_mora);
            return new_moras;
        }
    }
    moras
}

fn make_interrogative_mora(last_mora: MoraModel) -> MoraModel {
    let fix_vowel_length = 0.15;
    let adjust_pitch = 0.3;
    let max_pitch = 6.5;

    let mut pitch = last_mora.pitch + adjust_pitch;
    if pitch > max_pitch {
        pitch = max_pitch;
    }
    MoraModel {
        text: mora2text(last_mora.vowel.clone()),
        consonant: None,
        consonant_length: None,
        vowel: last_mora.vowel,
        vowel_length: fix_vowel_length,
        pitch,
    }
}

pub const DEFAULT_SAMPLING_RATE: u32 = 24000;

pub struct SynthesisEngine {
    openjtalk: OpenJTalk,
    core: VVCore,
}

impl SynthesisEngine {
    pub fn new(openjtalk: OpenJTalk, core: VVCore) -> SynthesisEngine {
        SynthesisEngine { openjtalk, core }
    }

    pub fn create_accent_phrases(
        &self,
        text: String,
        speaker_id: i64,
    ) -> Result<Vec<AccentPhraseModel>, String> {
        if text.is_empty() {
            return Ok(Vec::new());
        }

        let utterance = extract_fullcontext(self.openjtalk, text);
        if utterance.is_err() || utterance.as_ref().unwrap().breath_groups.is_empty() {
            return Ok(Vec::new());
        }
        let utterance = utterance.unwrap();

        let accent_phrases: Vec<AccentPhraseModel> = utterance
            .breath_groups
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut acc_vec, (i, breath_group)| {
                acc_vec.append(
                    &mut breath_group
                        .accent_phrases
                        .iter()
                        .enumerate()
                        .map(|(j, accent_phrase)| {
                            let moras = accent_phrase
                                .moras
                                .iter()
                                .map(|mora| {
                                    let mut moras_text = mora
                                        .phonemes()
                                        .iter()
                                        .map(|phoneme| phoneme.phoneme())
                                        .collect::<Vec<_>>()
                                        .join("");
                                    moras_text = moras_text.to_lowercase();
                                    if moras_text == "n" {
                                        moras_text = String::from("N");
                                    }
                                    let (consonant, consonant_length) = if mora.consonant.is_some()
                                    {
                                        (
                                            Some(mora.consonant.as_ref().unwrap().phoneme()),
                                            Some(0.0),
                                        )
                                    } else {
                                        (None, None)
                                    };
                                    MoraModel {
                                        text: moras_text,
                                        consonant,
                                        consonant_length,
                                        vowel: mora.vowel.phoneme(),
                                        vowel_length: 0.0,
                                        pitch: 0.0,
                                    }
                                })
                                .collect();

                            let pause_mora = if i != &utterance.breath_groups.len() - 1
                                && j == breath_group.accent_phrases.len() - 1
                            {
                                Some(MoraModel {
                                    text: "、".to_string(),
                                    consonant: None,
                                    consonant_length: None,
                                    vowel: "pau".to_string(),
                                    vowel_length: 0.0,
                                    pitch: 0.0,
                                })
                            } else {
                                None
                            };
                            AccentPhraseModel {
                                moras,
                                accent: accent_phrase.accent,
                                pause_mora,
                                is_interrogative: accent_phrase.is_interrogative,
                            }
                        })
                        .collect(),
                );
                acc_vec
            });

        self.replace_mora_data(accent_phrases, speaker_id)
    }

    pub fn replace_mora_data(
        &self,
        accent_phrases: Vec<AccentPhraseModel>,
        speaker_id: i64,
    ) -> Result<Vec<AccentPhraseModel>, String> {
        self.replace_mora_pitch(
            self.replace_phoneme_length(accent_phrases, speaker_id)?,
            speaker_id,
        )
    }

    pub fn replace_phoneme_length(
        &self,
        accent_phrases: Vec<AccentPhraseModel>,
        speaker_id: i64,
    ) -> Result<Vec<AccentPhraseModel>, String> {
        let mut accent_phrases = accent_phrases;
        let (_, phoneme_data_list) = SynthesisEngine::initial_process(&accent_phrases);

        let mut consonant_phoneme_list = Vec::new();
        let mut vowel_phoneme_list = Vec::new();
        let mut vowel_indexes_data = Vec::new();
        split_mora(
            &phoneme_data_list,
            &mut consonant_phoneme_list,
            &mut vowel_phoneme_list,
            &mut vowel_indexes_data,
        );

        let mut phoneme_list_s = phoneme_data_list
            .iter()
            .map(|phoneme_data| phoneme_data.phoneme_id())
            .collect::<Vec<_>>();
        let phoneme_length = if let Some(phoneme_length) =
            self.core.yukarin_s_forward(&mut phoneme_list_s, speaker_id)
        {
            phoneme_length
        } else {
            return Err(self.core.last_error_message());
        };

        let mut index = 0;
        for i in 0..accent_phrases.len() {
            let mut accent_phrase = accent_phrases[i].clone();
            let mut moras = accent_phrase.moras.clone();
            for j in 0..moras.len() {
                let mut mora = moras[j].clone();
                if mora.consonant.is_some() {
                    mora.consonant_length =
                        Some(phoneme_length[vowel_indexes_data[index + 1] as usize - 1]);
                }
                mora.vowel_length = phoneme_length[vowel_indexes_data[index + 1] as usize];
                index += 1;
                moras[j] = mora;
            }
            accent_phrase.moras = moras;
            if accent_phrase.pause_mora.is_some() {
                let mut pause_mora = accent_phrase.pause_mora.clone().unwrap();
                pause_mora.vowel_length = phoneme_length[vowel_indexes_data[index + 1] as usize];
                index += 1;
                accent_phrase.pause_mora = Some(pause_mora);
            }
            accent_phrases[i] = accent_phrase;
        }

        Ok(accent_phrases)
    }

    pub fn replace_mora_pitch(
        &self,
        accent_phrases: Vec<AccentPhraseModel>,
        speaker_id: i64,
    ) -> Result<Vec<AccentPhraseModel>, String> {
        let mut accent_phrases = accent_phrases;
        let (_, phoneme_data_list) = SynthesisEngine::initial_process(&accent_phrases);

        let mut base_start_accent_list = vec![0];
        let mut base_end_accent_list = vec![0];
        let mut base_start_accent_phrase_list = vec![0];
        let mut base_end_accent_phrase_list = vec![0];
        for accent_phrase in accent_phrases.iter() {
            let mut accent: u32 = if accent_phrase.accent == 1 { 0 } else { 1 };
            SynthesisEngine::create_one_accent_list(
                &mut base_start_accent_list,
                accent_phrase,
                accent as i32,
            );

            accent = accent_phrase.accent - 1;
            SynthesisEngine::create_one_accent_list(
                &mut base_end_accent_list,
                accent_phrase,
                accent as i32,
            );
            SynthesisEngine::create_one_accent_list(
                &mut base_start_accent_phrase_list,
                accent_phrase,
                0,
            );
            SynthesisEngine::create_one_accent_list(
                &mut base_end_accent_phrase_list,
                accent_phrase,
                -1,
            );
        }
        base_start_accent_list.push(0);
        base_end_accent_list.push(0);
        base_start_accent_phrase_list.push(0);
        base_end_accent_phrase_list.push(0);

        let mut consonant_phoneme_data_list = Vec::new();
        let mut vowel_phoneme_data_list = Vec::new();
        let mut vowel_indexes = Vec::new();
        split_mora(
            &phoneme_data_list,
            &mut consonant_phoneme_data_list,
            &mut vowel_phoneme_data_list,
            &mut vowel_indexes,
        );

        let mut consonant_phoneme_list = consonant_phoneme_data_list
            .iter()
            .map(|phoneme_data| phoneme_data.phoneme_id())
            .collect::<Vec<_>>();
        let mut vowel_phoneme_list = vowel_phoneme_data_list
            .iter()
            .map(|phoneme_data| phoneme_data.phoneme_id())
            .collect::<Vec<_>>();

        let mut start_accent_list = Vec::new();
        let mut end_accent_list = Vec::new();
        let mut start_accent_phrase_list = Vec::new();
        let mut end_accent_phrase_list = Vec::new();

        for vowel_index in vowel_indexes {
            start_accent_list.push(base_start_accent_list[vowel_index as usize]);
            end_accent_list.push(base_end_accent_list[vowel_index as usize]);
            start_accent_phrase_list.push(base_start_accent_phrase_list[vowel_index as usize]);
            end_accent_phrase_list.push(base_end_accent_phrase_list[vowel_index as usize]);
        }

        let f0_list = if let Some(f0_list) = self.core.yukarin_sa_forward(
            &mut vowel_phoneme_list,
            &mut consonant_phoneme_list,
            &mut start_accent_list,
            &mut end_accent_list,
            &mut start_accent_phrase_list,
            &mut end_accent_phrase_list,
            speaker_id,
        ) {
            f0_list
        } else {
            return Err(self.core.last_error_message());
        };

        let mut index = 0;
        for i in 0..accent_phrases.len() {
            let mut accent_phrase = accent_phrases[i].clone();
            let mut moras = accent_phrase.moras.clone();
            for j in 0..moras.len() {
                let mut mora = moras[j].clone();
                mora.pitch = f0_list[index + 1];
                index += 1;
                moras[j] = mora;
            }
            accent_phrase.moras = moras;
            if accent_phrase.pause_mora.is_some() {
                let mut pause_mora = accent_phrase.pause_mora.clone().unwrap();
                pause_mora.pitch = f0_list[index + 1];
                index += 1;
                accent_phrase.pause_mora = Some(pause_mora);
            }
            accent_phrases[i] = accent_phrase;
        }

        Ok(accent_phrases)
    }

    pub fn synthesis(
        &self,
        query: AudioQueryModel,
        speaker_id: i64,
        enable_interrogative_upspeak: bool,
    ) -> Result<Vec<f32>, String> {
        let AudioQueryModel {
            mut accent_phrases,
            speed_scale,
            pitch_scale,
            intonation_scale,
            pre_phoneme_length,
            post_phoneme_length,
            ..
        } = query;

        if enable_interrogative_upspeak {
            accent_phrases = adjust_interrogative_accent_phrases(&accent_phrases);
        }

        let (flatten_moras, phoneme_data_list) = SynthesisEngine::initial_process(&accent_phrases);

        let mut phoneme_length_list = vec![pre_phoneme_length];
        let mut f0_list = vec![0.0];
        let mut voiced = vec![false];
        let mut mean_f0 = 0.0;
        let mut count = 0;

        for mora in flatten_moras {
            let MoraModel {
                consonant,
                consonant_length,
                vowel_length,
                pitch,
                ..
            } = mora;
            if consonant.is_some() {
                phoneme_length_list.push(consonant_length.unwrap());
            }
            phoneme_length_list.push(vowel_length);
            let f0_single = pitch * 2.0_f32.powf(pitch_scale);
            f0_list.push(f0_single);
            let big_than_zero = f0_single > 0.0;
            voiced.push(big_than_zero);
            if big_than_zero {
                mean_f0 += f0_single;
                count += 1;
            }
        }
        phoneme_length_list.push(post_phoneme_length);
        f0_list.push(0.0);
        mean_f0 /= count as f32;

        if !mean_f0.is_nan() {
            for i in 0..voiced.len() {
                if voiced[i] {
                    f0_list[i] = (f0_list[i] - mean_f0) * intonation_scale + mean_f0;
                }
            }
        }

        let mut consonant_phoneme_data_list = Vec::new();
        let mut vowel_phoneme_data_list = Vec::new();
        let mut vowel_indexes = Vec::new();
        split_mora(
            &phoneme_data_list,
            &mut consonant_phoneme_data_list,
            &mut vowel_phoneme_data_list,
            &mut vowel_indexes,
        );

        let mut phoneme: Vec<Vec<f32>> = Vec::new();
        let mut f0: Vec<f32> = Vec::new();
        const RATE: f32 = 24000.0 / 256.0;
        let mut phoneme_length_sum = 0;
        let mut f0_count = 0;
        let mut vowel_indexes_index = 0;
        for i in 0..phoneme_length_list.len() {
            let phoneme_length =
                ((phoneme_length_list[i] * RATE).round() / speed_scale).round() as usize;
            let phoneme_id = phoneme_data_list[i].phoneme_id();
            for _ in 0..phoneme_length {
                let mut phonemes_vector = vec![0.0; OjtPhoneme::num_phoneme()];
                phonemes_vector[phoneme_id as usize] = 1.0;
                phoneme.push(phonemes_vector);
            }
            phoneme_length_sum += phoneme_length;
            if i as i64 == vowel_indexes[vowel_indexes_index] {
                for _ in 0..phoneme_length_sum {
                    f0.push(f0_list[f0_count]);
                }
                f0_count += 1;
                phoneme_length_sum = 0;
                vowel_indexes_index += 1;
            }
        }

        let mut flatten_phoneme = Vec::new();
        for mut p in phoneme {
            flatten_phoneme.append(&mut p);
        }

        if let Some(wave) = self.core.decode_forward(
            OjtPhoneme::num_phoneme(),
            &mut f0,
            &mut flatten_phoneme,
            speaker_id,
        ) {
            Ok(wave)
        } else {
            Err(self.core.last_error_message())
        }
    }

    pub fn synthesis_wave_format(
        &self,
        query: AudioQueryModel,
        speaker_id: i64,
        enable_interrogative_upspeak: bool,
    ) -> Result<Vec<u8>, String> {
        let wave = self.synthesis(query.clone(), speaker_id, enable_interrogative_upspeak)?;

        let AudioQueryModel {
            volume_scale,
            output_sampling_rate,
            output_stereo,
            ..
        } = query;

        let num_channels: u8 = if output_stereo { 2 } else { 1 };
        let bit_depth: u8 = 16;
        let repeat_count: u32 =
            (output_sampling_rate / DEFAULT_SAMPLING_RATE) * num_channels as u32;
        let block_size: u8 = bit_depth * num_channels / 8;

        let buf: Vec<u8> = Vec::new();
        let mut cur = Cursor::new(buf);
        cur.write("RIFF".as_bytes()).unwrap();
        let mut bytes_size = wave.len() as i32 * repeat_count as i32 * 8;
        let mut wave_size = bytes_size + 44 - 8;
        for _ in 0..4 {
            cur.write(&[(wave_size & 0xff) as u8]).unwrap(); // chunk size
            wave_size >>= 8;
        }
        cur.write("WAVEfmt ".as_bytes()).unwrap();

        cur.write(&[
            16,           // fmt header length
            0,            // fmt header length
            0,            // fmt header length
            0,            // fmt header length
            1,            // linear PCM
            0,            // linear PCM
            num_channels, // channel
            0,            // channel
        ])
        .unwrap();

        let mut sampling_rate = output_sampling_rate as i32;
        for _ in 0..4 {
            cur.write(&[(sampling_rate & 0xff) as u8]).unwrap();
            sampling_rate >>= 8;
        }
        let mut block_rate = (output_sampling_rate * block_size as u32) as i32;
        for _ in 0..4 {
            cur.write(&[(block_rate & 0xff) as u8]).unwrap();
            block_rate >>= 8;
        }

        cur.write(&[block_size, 0, bit_depth, 0]).unwrap();

        cur.write("data".as_bytes()).unwrap();
        let data_p = cur.position();
        for _ in 0..4 {
            cur.write(&[(bytes_size & 0xff) as u8]).unwrap();
            bytes_size >>= 8;
        }

        for value in wave {
            let mut v = value * volume_scale;
            // clip
            if v > 1.0 {
                v = 1.0;
            }
            if v < -1.0 {
                v = -1.0;
            }
            let data = (v * 0x7fff as f32) as i16;
            for _ in 0..repeat_count {
                let fst = (data & 0xff) as u8;
                #[allow(overflowing_literals)]
                let snd = ((data & 0xff00) >> 8) as u8;
                cur.write(&[fst, snd]).unwrap();
            }
        }

        let mut last_p = cur.position();
        let last_p_tmp = last_p;
        last_p -= 8;
        cur.seek(SeekFrom::Start(4)).unwrap();
        for _ in 0..4 {
            cur.write(&[(last_p & 0xff) as u8]).unwrap();
            last_p >>= 8;
        }
        last_p = last_p_tmp;
        cur.seek(SeekFrom::Start(data_p)).unwrap();
        let mut pointer = last_p - data_p - 4;
        for _ in 0..4 {
            cur.write(&[(pointer & 0xff) as u8]).unwrap();
            pointer >>= 8;
        }

        Ok(cur.into_inner())
    }

    pub fn finalize(&self) {
        self.core.finalize();
    }

    fn initial_process(
        accent_phrases: &Vec<AccentPhraseModel>,
    ) -> (Vec<MoraModel>, Vec<OjtPhoneme>) {
        let flatten_moras = to_flatten_mora(accent_phrases);

        let mut phoneme_str_list = vec!["pau".to_string()];
        for mora in flatten_moras.iter() {
            if let Some(consonant) = mora.consonant.clone() {
                phoneme_str_list.push(consonant);
            }
            phoneme_str_list.push(mora.vowel.clone());
        }
        phoneme_str_list.push("pau".to_string());

        let phoneme_data_list = to_phoneme_data_list(phoneme_str_list.clone());

        (flatten_moras, phoneme_data_list)
    }

    fn create_one_accent_list(
        accent_list: &mut Vec<i64>,
        accent_phrase: &AccentPhraseModel,
        point: i32,
    ) {
        let mut one_accent_list = Vec::new();

        for (i, mora) in accent_phrase.moras.iter().enumerate() {
            let value = if i as i32 == point
                || (point < 0 && i == (accent_phrase.moras.len() as i32 + point) as usize)
            {
                1
            } else {
                0
            };
            one_accent_list.push(value as i64);
            if mora.consonant.is_some() {
                one_accent_list.push(value as i64);
            }
        }
        if accent_phrase.pause_mora.is_some() {
            one_accent_list.push(0);
        }
        accent_list.append(&mut one_accent_list);
    }
}

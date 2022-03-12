use super::{mora::Mora, phoneme::Phoneme};

#[derive(Debug, Clone)]
pub struct AccentPhrase {
    pub moras: Vec<Mora>,
    pub accent: u32,
    pub is_interrogative: bool,
}

impl AccentPhrase {
    pub fn new(moras: Vec<Mora>, accent: u32, is_interrogative: bool) -> AccentPhrase {
        AccentPhrase {
            moras,
            accent,
            is_interrogative,
        }
    }

    pub fn from_phonemes(phonemes: Vec<Phoneme>) -> Result<Self, String> {
        let mut moras = Vec::new();
        let mut mora_phonemes = Vec::new();

        for i in 0..phonemes.len() {
            // workaround for Hihosiba/voicevox_engine#57
            if phonemes[i].contexts.get("a2").unwrap() == "49" {
                break;
            }
            mora_phonemes.push(phonemes[i].clone());

            if phonemes.len() == i + 1
                || phonemes[i].contexts.get("a2").unwrap()
                    != phonemes[i + 1].contexts.get("a2").unwrap()
            {
                let mora = if mora_phonemes.len() == 1 {
                    Mora::from_vowel(mora_phonemes[0].clone())
                } else if mora_phonemes.len() == 2 {
                    Mora::from_consonant_vowel(mora_phonemes[0].clone(), mora_phonemes[1].clone())
                } else {
                    return Err("mora_phonemes size is not 1 or 2".to_string());
                };
                moras.push(mora);
                mora_phonemes.clear();
            }
        }

        // workaround for VOICEVOX/voicevox_engine#55
        let accent = std::cmp::min(
            moras[0]
                .vowel
                .contexts
                .get("f2")
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            moras.len() as u32,
        );
        let is_interrogative = moras[moras.len() - 1].vowel.contexts.get("f3").unwrap() == "1";
        Ok(Self {
            moras,
            accent,
            is_interrogative,
        })
    }

    pub fn set_context(&mut self, key: String, value: String) {
        for mora in self.moras.iter_mut() {
            mora.set_context(key.clone(), value.clone());
        }
    }

    pub fn phonemes(&self) -> Vec<Phoneme> {
        let mut phonemes = Vec::new();
        for mora in self.moras.iter() {
            phonemes.append(&mut mora.phonemes());
        }
        phonemes
    }

    pub fn labels(&self) -> Vec<String> {
        self.phonemes()
            .iter()
            .map(|phoneme| phoneme.label.clone())
            .collect()
    }

    pub fn merge(&self, accent_phrase: &AccentPhrase) -> AccentPhrase {
        let mut moras = Vec::new();
        moras.append(&mut self.moras.clone());
        moras.append(&mut accent_phrase.moras.clone());
        AccentPhrase::new(moras, self.accent, self.is_interrogative)
    }
}

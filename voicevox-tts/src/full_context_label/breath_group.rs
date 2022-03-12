use super::{accent_phrase::AccentPhrase, phoneme::Phoneme};

#[derive(Debug, Clone)]
pub struct BreathGroup {
    pub accent_phrases: Vec<AccentPhrase>,
}

impl BreathGroup {
    pub fn new(accent_phrases: Vec<AccentPhrase>) -> BreathGroup {
        BreathGroup { accent_phrases }
    }

    pub fn from_phonemes(phonemes: Vec<Phoneme>) -> Result<BreathGroup, String> {
        let mut accent_phrases = Vec::new();
        let mut accent_phonemes = Vec::new();

        for i in 0..phonemes.len() {
            accent_phonemes.push(phonemes[i].clone());

            if i + 1 == phonemes.len()
                || phonemes[i].contexts.get("i3").unwrap()
                    != phonemes[i + 1].contexts.get("i3").unwrap()
                || phonemes[i].contexts.get("f5").unwrap()
                    != phonemes[i + 1].contexts.get("f5").unwrap()
            {
                accent_phrases.push(AccentPhrase::from_phonemes(accent_phonemes.clone())?);
                accent_phonemes.clear();
            }
        }

        Ok(BreathGroup::new(accent_phrases))
    }

    pub fn set_context(&mut self, key: String, value: String) {
        for accent_phrase in self.accent_phrases.iter_mut() {
            accent_phrase.set_context(key.clone(), value.clone());
        }
    }

    pub fn phonemes(&self) -> Vec<Phoneme> {
        let mut phonemes = Vec::new();
        for accent_phrase in self.accent_phrases.iter() {
            phonemes.append(&mut accent_phrase.phonemes());
        }
        phonemes
    }

    pub fn labels(&self) -> Vec<String> {
        self.phonemes()
            .iter()
            .map(|phoneme| phoneme.label.clone())
            .collect()
    }
}

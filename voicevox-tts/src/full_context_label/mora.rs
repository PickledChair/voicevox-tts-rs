use super::phoneme::Phoneme;

#[derive(Debug, Clone)]
pub struct Mora {
    pub consonant: Option<Phoneme>,
    pub vowel: Phoneme,
}

impl Mora {
    pub fn from_consonant_vowel(consonant: Phoneme, vowel: Phoneme) -> Mora {
        Mora { consonant: Some(consonant), vowel }
    }

    pub fn from_vowel(vowel: Phoneme) -> Mora {
        Mora { consonant: None, vowel }
    }

    pub fn set_context(&mut self, key: String, value: String) {
        if let Some(ref mut consonant) = self.consonant {
            consonant.contexts.insert(key.clone(), value.clone());
        }
        self.vowel.contexts.insert(key, value);
    }

    pub fn phonemes(&self) -> Vec<Phoneme> {
        let mut phonemes = Vec::new();
        if self.consonant.is_some() {
            phonemes.push(self.consonant.as_ref().unwrap().clone());
        }
        phonemes.push(self.vowel.clone());
        phonemes
    }

    pub fn labels(&self) -> Vec<String> {
        let mut labels = Vec::new();
        for phoneme in self.phonemes() {
            labels.push(phoneme.label);
        }
        labels
    }
}

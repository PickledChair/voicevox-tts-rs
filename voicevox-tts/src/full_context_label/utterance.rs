use super::{breath_group::BreathGroup, phoneme::Phoneme};

#[derive(Debug, Clone)]
pub struct Utterance {
    pub breath_groups: Vec<BreathGroup>,
    pub pauses: Vec<Phoneme>,
}

impl Utterance {
    pub fn new(breath_groups: Vec<BreathGroup>, pauses: Vec<Phoneme>) -> Utterance {
        Utterance {
            breath_groups,
            pauses,
        }
    }

    pub fn from_phonemes(phonemes: Vec<Phoneme>) -> Result<Utterance, String> {
        let mut breath_groups = Vec::new();
        let mut group_phonemes = Vec::new();
        let mut pauses = Vec::new();

        for phoneme in phonemes {
            if !phoneme.is_pause() {
                group_phonemes.push(phoneme);
            } else {
                pauses.push(phoneme);

                if !group_phonemes.is_empty() {
                    breath_groups.push(BreathGroup::from_phonemes(group_phonemes.clone())?);
                    group_phonemes.clear();
                }
            }
        }
        Ok(Utterance::new(breath_groups, pauses))
    }

    pub fn set_context(&mut self, key: String, value: String) {
        for breath_group in self.breath_groups.iter_mut() {
            breath_group.set_context(key.clone(), value.clone());
        }
    }

    pub fn phonemes(&self) -> Vec<Phoneme> {
        let mut accent_phrases = Vec::new();
        for breath_group in self.breath_groups.iter() {
            accent_phrases.append(&mut breath_group.accent_phrases.clone());
        }

        let mut phonemes = Vec::new();
        for i in 0..self.pauses.len() {
            phonemes.push(self.pauses[i].clone());

            if i < self.pauses.len() - 1 {
                phonemes.append(&mut self.breath_groups[i].phonemes().clone());
            }
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

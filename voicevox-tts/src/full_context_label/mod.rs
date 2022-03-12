pub mod accent_phrase;
pub mod breath_group;
pub mod mora;
pub mod phoneme;
pub mod utterance;

use openjtalk::OpenJTalk;
use phoneme::Phoneme;
use utterance::Utterance;

pub fn extract_fullcontext(openjtalk: OpenJTalk, text: String) -> Result<Utterance, String> {
    let labels = openjtalk.extract_fullcontext(text);
    let phonemes = labels
        .into_iter()
        .map(|label| Phoneme::from_label(label).unwrap())
        .collect();
    Utterance::from_phonemes(phonemes)
}

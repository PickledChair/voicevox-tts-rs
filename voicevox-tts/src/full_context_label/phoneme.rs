use std::collections::HashMap;

use regex::Regex;

fn string_feature_by_regex(pattern: &str, label: &str) -> Option<String> {
    let re = Regex::new(pattern).ok()?;
    if let Some(caps) = re.captures(label) {
        Some(caps.get(2)?.as_str().to_string())
    } else {
        None
    }
}

#[derive(Debug, Clone)]
pub struct Phoneme {
    pub contexts: HashMap<String, String>,
    pub label: String,
}

impl Phoneme {
    pub fn new(contexts: HashMap<String, String>, label: String) -> Phoneme {
        Phoneme { contexts, label }
    }

    pub fn from_label(label: String) -> Option<Self> {
        let mut contexts = HashMap::new();

        contexts.insert(
            "p3".to_string(),
            string_feature_by_regex(r"(\-(.*?)\+)", &label)?,
        );
        contexts.insert(
            "a2".to_string(),
            string_feature_by_regex(r"(\+(\d+|xx)\+)", &label)?,
        );
        contexts.insert(
            "a3".to_string(),
            string_feature_by_regex(r"(\+(\d+|xx)/B:)", &label)?,
        );
        contexts.insert(
            "f1".to_string(),
            string_feature_by_regex(r"(/F:(\d+|xx)_)", &label)?,
        );
        contexts.insert(
            "f2".to_string(),
            string_feature_by_regex(r"(_(\d+|xx)#)", &label)?,
        );
        contexts.insert(
            "f3".to_string(),
            string_feature_by_regex(r"(#(\d+|xx)_)", &label)?,
        );
        contexts.insert(
            "f5".to_string(),
            string_feature_by_regex(r"(@(\d+|xx)_)", &label)?,
        );
        contexts.insert(
            "h1".to_string(),
            string_feature_by_regex(r"(/H:(\d+|xx)_)", &label)?,
        );
        contexts.insert(
            "i3".to_string(),
            string_feature_by_regex(r"(@(\d+|xx)\+)", &label)?,
        );
        contexts.insert(
            "j1".to_string(),
            string_feature_by_regex(r"(/J:(\d+|xx)_)", &label)?,
        );

        Some(Self::new(contexts, label))
    }

    pub fn phoneme(&self) -> String {
        self.contexts.get("p3").unwrap().clone()
    }

    pub fn is_pause(&self) -> bool {
        self.contexts.get("f1").unwrap() == "xx"
    }
}

#[cfg(test)]
mod phoneme_tests {
    use super::Phoneme;

    fn get_phoneme() -> Phoneme {
        Phoneme::from_label(
            "a^sh-I+t=a/A:-1+2+3/B:xx-xx_xx/C:02_xx+xx/D:23+xx_xx/E:xx_xx!xx_xx-xx/F:4_3#0_xx@1_5|1_19/G:4_1%0_xx_1/H:xx_xx/I:5-19@1+1&1-5|1+19/J:xx_xx/K:1+5-19"
                .to_string()
        ).unwrap()
    }

    #[test]
    fn test_phoneme_is_not_pause() {
        assert!(!get_phoneme().is_pause());
    }
}

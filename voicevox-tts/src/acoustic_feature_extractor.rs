use once_cell::sync::OnceCell;
use std::{collections::HashMap, sync::Mutex};

macro_rules! make_phoneme_map {
    ($(($key:expr, $value:expr)),*) => {
        {
            let mut m = HashMap::new();
            $(m.insert($key.to_string(), $value);)*
            m
        }
    };
}

#[derive(Debug, Clone, Default)]
pub struct OjtPhoneme {
    pub phoneme: String,
    pub start: f32,
    pub end: f32,
}

impl OjtPhoneme {
    pub fn new(phoneme: String, start: f32, end: f32) -> OjtPhoneme {
        OjtPhoneme {
            phoneme,
            start,
            end,
        }
    }

    #[rustfmt::skip]
    pub fn phoneme_map() -> &'static Mutex<HashMap<String, i32>> {
        static PHONEME_MAP: OnceCell<Mutex<HashMap<String, i32>>> = OnceCell::new();
        PHONEME_MAP.get_or_init(|| {
            let m = make_phoneme_map!(
                ("pau", 0), ("A", 1),   ("E", 2),   ("I", 3),   ("N", 4),   ("O", 5),   ("U", 6),   ("a", 7),   ("b", 8),
                ("by", 9),  ("ch", 10), ("cl", 11), ("d", 12),  ("dy", 13), ("e", 14),  ("f", 15),  ("g", 16),  ("gw", 17),
                ("gy", 18), ("h", 19),  ("hy", 20), ("i", 21),  ("j", 22),  ("k", 23),  ("kw", 24), ("ky", 25), ("m", 26),
                ("my", 27), ("n", 28),  ("ny", 29), ("o", 30),  ("p", 31),  ("py", 32), ("r", 33),  ("ry", 34), ("s", 35),
                ("sh", 36), ("t", 37),  ("ts", 38), ("ty", 39), ("u", 40),  ("v", 41),  ("w", 42),  ("y", 43),  ("z", 44)
            );
            Mutex::new(m)
        })
    }

    pub fn num_phoneme() -> usize {
        loop {
            if let Ok(map) = OjtPhoneme::phoneme_map().lock() {
                return map.len();
            }
        }
    }

    pub fn space_phoneme() -> String {
        String::from("pau")
    }

    pub fn phoneme_id(&self) -> i64 {
        if self.phoneme.is_empty() {
            -1
        } else {
            loop {
                if let Ok(map) = OjtPhoneme::phoneme_map().lock() {
                    return *map.get(&self.phoneme).unwrap() as i64;
                }
            }
        }
    }

    pub fn convert(phonemes: Vec<OjtPhoneme>) -> Vec<OjtPhoneme> {
        let mut phonemes = phonemes;
        if !phonemes[0].phoneme.contains("sil") {
            phonemes[0].phoneme = OjtPhoneme::space_phoneme();
        }
        let phonemes_len = phonemes.len();
        if !phonemes[phonemes_len - 1].phoneme.contains("sil") {
            phonemes[phonemes_len - 1].phoneme = OjtPhoneme::space_phoneme();
        }
        phonemes
    }
}

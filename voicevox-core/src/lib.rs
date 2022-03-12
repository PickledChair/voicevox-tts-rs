use voicevox_core_sys::core;

use std::ffi::{CStr, CString};
use std::fmt;
use std::path::Path;

pub struct VVCore {
    core_lib: core,
}

impl fmt::Debug for VVCore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("VVCore").field("core_lib", &"???").finish()
    }
}

impl VVCore {
    pub fn new(library_path: &Path) -> Result<Self, String> {
        match unsafe { core::new(library_path) } {
            Ok(core_lib) => Ok(Self { core_lib }),
            Err(err) => Err(format!("Error: {}", err)),
        }
    }

    pub fn initialize(&self, root_dir_path: &Path, use_gpu: bool, cpu_num_threads: i32) -> bool {
        let root_dir_path = CString::new(format!("{}", root_dir_path.display())).unwrap();
        unsafe {
            self.core_lib
                .initialize(root_dir_path.as_ptr(), use_gpu, cpu_num_threads)
        }
    }

    pub fn finalize(&self) {
        unsafe {
            self.core_lib.finalize();
        }
    }

    pub fn metas(&self) -> String {
        let metas_c_str = unsafe { CStr::from_ptr(self.core_lib.metas()) };
        std::str::from_utf8(metas_c_str.to_bytes())
            .unwrap()
            .to_string()
    }

    pub fn supported_devices(&self) -> String {
        let devs_c_str = unsafe { CStr::from_ptr(self.core_lib.supported_devices()) };
        std::str::from_utf8(devs_c_str.to_bytes())
            .unwrap()
            .to_string()
    }

    pub fn yukarin_s_forward(&self, phoneme_list: &mut [i64], speaker_id: i64) -> Option<Vec<f32>> {
        let length = phoneme_list.len();
        let mut output = Vec::with_capacity(length);

        let success = unsafe {
            self.core_lib.yukarin_s_forward(
                length as i64,
                phoneme_list.as_mut_ptr(),
                [speaker_id].as_mut_ptr(),
                output.as_mut_ptr(),
            )
        };

        if success {
            unsafe {
                output.set_len(length);
            }
            Some(output)
        } else {
            None
        }
    }

    pub fn yukarin_sa_forward(
        &self,
        vowel_phoneme_list: &mut [i64],
        consonant_phoneme_list: &mut [i64],
        start_accent_list: &mut [i64],
        end_accent_list: &mut [i64],
        start_accent_phrase_list: &mut [i64],
        end_accent_phrase_list: &mut [i64],
        speaker_id: i64,
    ) -> Option<Vec<f32>> {
        let length = vowel_phoneme_list.len();
        let mut output = Vec::with_capacity(length);

        let success = unsafe {
            self.core_lib.yukarin_sa_forward(
                length as i64,
                vowel_phoneme_list.as_mut_ptr(),
                consonant_phoneme_list.as_mut_ptr(),
                start_accent_list.as_mut_ptr(),
                end_accent_list.as_mut_ptr(),
                start_accent_phrase_list.as_mut_ptr(),
                end_accent_phrase_list.as_mut_ptr(),
                [speaker_id].as_mut_ptr(),
                output.as_mut_ptr(),
            )
        };

        if success {
            unsafe {
                output.set_len(length);
            }
            Some(output)
        } else {
            None
        }
    }

    pub fn decode_forward(
        &self,
        phoneme_size: usize,
        f0: &mut [f32],
        phoneme: &mut [f32],
        speaker_id: i64,
    ) -> Option<Vec<f32>> {
        let length = f0.len();
        let mut output = Vec::with_capacity(length * 256);

        let success = unsafe {
            self.core_lib.decode_forward(
                length as i64,
                phoneme_size as i64,
                f0.as_mut_ptr(),
                phoneme.as_mut_ptr(),
                [speaker_id].as_mut_ptr(),
                output.as_mut_ptr(),
            )
        };

        if success {
            unsafe {
                output.set_len(length * 256);
            }
            Some(output)
        } else {
            None
        }
    }

    pub fn last_error_message(&self) -> String {
        let err_msg_c_str = unsafe { CStr::from_ptr(self.core_lib.last_error_message()) };
        std::str::from_utf8(err_msg_c_str.to_bytes())
            .unwrap()
            .to_string()
    }
}

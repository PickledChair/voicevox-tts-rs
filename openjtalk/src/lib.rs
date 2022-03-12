use openjtalk_sys::*;

use std::ffi::CString;
use std::os::raw::c_void;
use std::path::Path;

#[derive(Clone, Copy)]
pub struct OpenJTalk {
    ptr: *mut c_void,
}

impl OpenJTalk {
    pub fn new(dn_mecab: &Path) -> Result<Self, String> {
        let ojt = Self {
            ptr: unsafe { OpenJTalk_create() },
        };
        ojt.load(dn_mecab)?;
        Ok(ojt)
    }

    pub fn load(&self, dn_mecab: &Path) -> Result<(), String> {
        let dn_mecab_cstr = CString::new(format!("{}", dn_mecab.display())).unwrap();
        let res = unsafe { OpenJTalk_load(self.ptr, dn_mecab_cstr.as_ptr()) };
        if res == 0 {
            Ok(())
        } else {
            Err(format!(
                "Mecab load error: couldn't load mecab dictionary: {}",
                dn_mecab.display()
            ))
        }
    }

    pub fn extract_fullcontext<T: AsRef<str>>(&self, text: T) -> Vec<String> {
        let text = CString::new(text.as_ref()).unwrap();
        let box_size = Box::new(0);
        let size_ptr = Box::into_raw(box_size);
        let mut result = Vec::new();
        unsafe {
            let labels_ptr = OpenJTalk_extract_fullcontext(self.ptr, text.as_ptr(), size_ptr);
            let box_size = Box::from_raw(size_ptr);
            for ptr in std::slice::from_raw_parts(labels_ptr, *box_size as usize) {
                let c_str = CString::from_raw(*ptr);
                result.push(c_str.to_str().unwrap().to_string());
            }
        }
        return result;
    }

    pub fn clear(&self) {
        unsafe {
            OpenJTalk_clear(self.ptr);
        }
    }

    pub fn delete(&self) {
        unsafe {
            OpenJTalk_delete(self.ptr);
        }
    }
}

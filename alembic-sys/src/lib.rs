mod generated;
pub use self::generated::*;
use std::{os::raw::c_char, path::Path, str::Utf8Error};

//--------------------------------------------------------------------------------------------------
pub trait AsStringRef {
    fn as_stringref(&self) -> CStringRef;
}

impl AsStringRef for Path {
    fn as_stringref(&self) -> CStringRef {
        let path_str = self
            .as_os_str()
            .to_str()
            .expect("non-utf8 characters in path");
        CStringRef {
            len: path_str.len(),
            ptr: path_str.as_ptr() as *const c_char,
        }
    }
}

impl AsStringRef for str {
    fn as_stringref(&self) -> CStringRef {
        CStringRef {
            len: self.len(),
            ptr: self.as_ptr() as *const c_char,
        }
    }
}

impl CStringRef {
    // warning: unbounded lifetime, extra unsafe
    pub unsafe fn try_into_str<'a>(self) -> Result<&'a str, Utf8Error> {
        unsafe {
            let slice = std::slice::from_raw_parts(self.ptr as *const u8, self.len);
            std::str::from_utf8(slice)
        }
    }
}

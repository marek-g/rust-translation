use winapi::ctypes::c_int;
use winapi::um::winnt::LCID;
use winapi::um::winnls::LCTYPE;
use winapi::um::winnt::WCHAR;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use crate::locale::Locale;

impl Locale {
    pub fn current() -> Option<Self> {
		let langid = unsafe { winapi::um::winnls::GetUserDefaultUILanguage() };

		let mut lang_buf: [WCHAR; 9] = [0; 9];
		const LOCALE_SISO639LANGNAME: LCTYPE = 0x59;
		let buf_size = unsafe {
			winapi::um::winnls::GetLocaleInfoW(langid as LCID,
				LOCALE_SISO639LANGNAME,
				lang_buf.as_mut_ptr(), lang_buf.len() as c_int)
		} as usize;

		if buf_size > 0 {
			if let Some(lang) = OsString::from_wide(&lang_buf[..buf_size - 1]).into_string().ok() {
				return Locale::from(&lang);
			}
		}

        None
    }
}

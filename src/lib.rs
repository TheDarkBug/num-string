mod conversion;
mod interface;

#[allow(non_snake_case)]
pub mod android {
    use std::ffi::{c_char, CStr, CString};

    use crate::conversion;
    use crate::interface;

    #[no_mangle]
    pub unsafe extern "C" fn convert(
        inputc_str: *const c_char,
        langc_str: *const c_char,
    ) -> *const c_char {
        let input = CStr::from_ptr(inputc_str).to_str().unwrap();
        let lang = CStr::from_ptr(langc_str).to_str().unwrap();
        if lang.is_empty() {
            return CString::new("").unwrap().into_raw();
        }
        let digits = interface::Digits::new(&lang);
        let output = &conversion::convert(&digits, conversion::separate_nums(input));
        return CString::new(&**output).unwrap().into_raw();
    }
}

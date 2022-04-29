#[cfg(target_os="android")]
mod conversion;
#[cfg(target_os="android")]
mod interface;

#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::objects::{JClass, JString};
    use self::jni::sys::jstring;
    use self::jni::JNIEnv;
    use std::ffi::CStr;

    use crate::conversion::*;
    use crate::interface::*;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_tdb_numstring_MainActivity_convert(
        env: JNIEnv,
        _class: JClass,
        j_input: JString,
        j_lang: JString,
    ) -> jstring {
        let lang = CStr::from_ptr(env.get_string(j_lang).unwrap().get_raw()).to_string_lossy().into_owned();
        let input = CStr::from_ptr(env.get_string(j_input).unwrap().get_raw()).to_string_lossy().into_owned();
        let digits = Digits::new(&lang);
        // let output = format!("Out: {}\n\n\n{}", lang, input);//&convert(&digits, separate_nums(input));
        let output = &convert(&digits, separate_nums(&input));
        env.new_string(&output)
            .expect("Couldn't create java string!")
            .into_inner()
    }
}

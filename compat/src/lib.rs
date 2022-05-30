use windows::core::PWSTR;

pub fn to_wstring(value: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

pub unsafe fn pwstr_to_string(ptr: PWSTR) -> String {
    use std::slice::from_raw_parts;
    let len = (0usize..)
        .find(|&n| *ptr.0.add(n) == 0)
        .expect("Couldn't find null terminator");
    let array: &[u16] = from_raw_parts(ptr.0, len);
    String::from_utf16_lossy(array)
}

pub fn string_from_pwstr(source: PWSTR) -> Option<String> {
    if source.0.is_null() {
        None
    } else {
        let mut buffer = Vec::new();
        let mut pwz = source.0;

        unsafe {
            while *pwz != 0 {
                buffer.push(*pwz);
                pwz = pwz.add(1);
            }
        }

        Some(String::from_utf16(&buffer).expect("string_from_pwstr"))
    }
}

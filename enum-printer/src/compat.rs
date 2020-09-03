use winapi::shared::ntdef::PWSTR;

#[allow(dead_code)]
pub fn to_wstring(value: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

#[allow(dead_code)]
pub unsafe fn pwstr_to_string(ptr: PWSTR) -> String {
    use std::slice::from_raw_parts;
    let len = (0usize..)
        .find(|&n| *ptr.offset(n as isize) == 0)
        .expect("Couldn't find null terminator");
    let array: &[u16] = from_raw_parts(ptr, len);
    String::from_utf16_lossy(array)
}

extern crate winapi;

use std::io::Error;

fn to_wstring(value: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

#[cfg(windows)]
fn show_message_box(title: &str, msg: &str) -> Result<i32, Error> {
    use std::ptr::null_mut;
    use winapi::um::winuser::{MessageBoxW, MB_ICONINFORMATION, MB_OK};

    let msg_w = to_wstring(msg);
    let title_w = to_wstring(title);
    let ret = unsafe {
        MessageBoxW(
            null_mut(),
            msg_w.as_ptr(),
            title_w.as_ptr(),
            MB_OK | MB_ICONINFORMATION,
        )
    };
    if ret == 0 {
        Err(Error::last_os_error())
    } else {
        Ok(ret)
    }
}

#[cfg(windows)]
fn main() -> Result<(), Error> {
    let ret = show_message_box("Greeting from Rust.", "Hello Win32 API.")?;
    println!("{}", ret);
    Ok(())
}

#[cfg(not(windows))]
fn main() {
    println!("This program only work on Windows.")
}

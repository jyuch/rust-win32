use compat::to_wstring;
use std::io::Error;
use windows::core::PCWSTR;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONINFORMATION, MB_OK};

fn show_message_box(title: &str, msg: &str) -> Result<i32, Error> {
    let msg_w = to_wstring(msg);
    let title_w = to_wstring(title);
    let ret = unsafe {
        MessageBoxW(
            None,
            PCWSTR(msg_w.as_ptr()),
            PCWSTR(title_w.as_ptr()),
            MB_OK | MB_ICONINFORMATION,
        )
    };
    if ret.0 == 0 {
        Err(Error::last_os_error())
    } else {
        Ok(ret.0)
    }
}

fn main() -> Result<(), Error> {
    let ret = show_message_box("Greeting from Rust.", "Hello Win32 API.")?;
    println!("{}", ret);
    Ok(())
}

use crate::compat::to_wstring;
use winapi::_core::ptr::null_mut;
use winapi::ctypes::{c_int, c_void};
use winapi::shared::windef::{HDC, HFONT};
use winapi::um::wingdi::{
    CreateDCW, CreateFontW, DeleteDC, DeleteObject, EndDoc, EndPage, SelectObject, StartDocW,
    StartPage, TextOutW, CLIP_DEFAULT_PRECIS, DEFAULT_QUALITY, DOCINFOW, FF_ROMAN, FW_BOLD,
    OUT_DEFAULT_PRECIS, SHIFTJIS_CHARSET, VARIABLE_PITCH,
};

mod compat;

unsafe fn create_font(font_family: &str, font_height: u32) -> HFONT {
    CreateFontW(
        font_height as c_int,
        0,
        0,
        0,
        FW_BOLD,
        0,
        0,
        0,
        SHIFTJIS_CHARSET,
        OUT_DEFAULT_PRECIS,
        CLIP_DEFAULT_PRECIS,
        DEFAULT_QUALITY,
        VARIABLE_PITCH | FF_ROMAN,
        to_wstring(font_family).as_ptr(),
    )
}

fn print(printer_name: &str, driver_name: &str) {
    let printer_name = to_wstring(printer_name);
    let driver_name = to_wstring(driver_name);

    let hdc: HDC = unsafe {
        CreateDCW(
            driver_name.as_ptr(),
            printer_name.as_ptr(),
            null_mut(),
            null_mut(),
        )
    };

    let mut doc_info = DOCINFOW::default();
    doc_info.cbSize = std::mem::size_of::<DOCINFOW>() as c_int;
    doc_info.lpszDocName = to_wstring("Print by rust").as_ptr();

    let font = unsafe { create_font("ＭＳ ゴシック", 200) };

    unsafe {
        StartDocW(hdc, &mut doc_info);
        StartPage(hdc);
        SelectObject(hdc, font as *mut c_void);
        TextOutW(hdc, 200, 200, to_wstring("テスト印刷").as_ptr(), 5);
        EndPage(hdc);
        EndDoc(hdc);
    }

    unsafe { DeleteObject(font as *mut c_void) };
    unsafe { DeleteDC(hdc) };
}

fn main() {
    print("Microsoft Print to PDF", "Microsoft Print To PDF");
}

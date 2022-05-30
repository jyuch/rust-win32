use compat::{pwstr_to_string, string_from_pwstr};
use std::alloc::{alloc, dealloc, Layout};
use std::ffi::c_void;
use std::io::Error;
use std::ptr::{null, null_mut};
use std::slice::from_raw_parts;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Graphics::Printing::{EnumPrintersW, PRINTER_ENUM_LOCAL, PRINTER_INFO_2W};
use windows::Win32::System::Memory::{
    VirtualAlloc, VirtualFree, MEM_COMMIT, MEM_RELEASE, PAGE_READWRITE,
};

fn main() {
    let mut needed: u32 = 0;
    let mut returned: u32 = 0;

    unsafe {
        EnumPrintersW(
            PRINTER_ENUM_LOCAL,
            None,
            2,
            null_mut(),
            0,
            &mut needed,
            &mut returned,
        );
    }

    let layout = Layout::array::<u8>(needed as usize).expect("Failed to create layout.");

    unsafe {
        let printer_enum_pointer = alloc(layout);
        EnumPrintersW(
            PRINTER_ENUM_LOCAL,
            None,
            2,
            printer_enum_pointer,
            needed,
            &mut needed,
            &mut returned,
        );

        let arr: &[PRINTER_INFO_2W] = std::slice::from_raw_parts(
            printer_enum_pointer as *mut PRINTER_INFO_2W,
            returned as usize,
        );

        for it in arr {
            let name = string_from_pwstr(it.pPrinterName).unwrap();
            let driver = string_from_pwstr(it.pDriverName).unwrap();
            println!("Name:{} Driver:{}", name, driver);
        }

        dealloc(printer_enum_pointer, layout);
    }
}

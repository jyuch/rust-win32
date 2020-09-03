use crate::compat::pwstr_to_string;
use std::io::Error;
use winapi::_core::ptr::null_mut;
use winapi::_core::slice::from_raw_parts;
use winapi::shared::minwindef::{BOOL, DWORD};
use winapi::um::winspool::{EnumPrintersW, PRINTER_ENUM_LOCAL, PRINTER_INFO_2W};

mod compat;

#[derive(Debug)]
struct Printer {
    name: String,
    driver: String,
}

impl Printer {
    fn new(name: &str, driver: &str) -> Printer {
        Self {
            name: name.to_string(),
            driver: driver.to_string(),
        }
    }
}

fn get_printers() -> Result<Vec<Printer>, Error> {
    let mut needed: DWORD = 0 as DWORD;
    let mut returned: DWORD = 0 as DWORD;

    unsafe {
        EnumPrintersW(
            PRINTER_ENUM_LOCAL,
            null_mut(),
            2,
            null_mut(),
            0,
            &mut needed,
            &mut returned,
        )
    };

    // とりあえずはメモリは投げ捨てるスタイルでやる
    let printer = unsafe { libc::malloc(needed as libc::size_t) as *mut PRINTER_INFO_2W };

    let ret: BOOL = unsafe {
        EnumPrintersW(
            PRINTER_ENUM_LOCAL,
            null_mut(),
            2,
            printer as *mut u8,
            needed,
            &mut needed,
            &mut returned,
        )
    };

    if ret == 0 {
        return Err(Error::last_os_error());
    }

    let mut printers = Vec::<Printer>::with_capacity(returned as usize);
    unsafe {
        let array = from_raw_parts(printer, returned as usize);

        for it in array {
            let printer_name = pwstr_to_string(it.pPrinterName);
            let driver_name = pwstr_to_string(it.pDriverName);
            printers.push(Printer::new(printer_name.as_str(), driver_name.as_str()));
        }
    }

    Ok(printers)
}

fn main() {
    let printers = get_printers().expect("Failed to get printer info");
    for it in printers {
        println!("{:?}", it);
    }
}

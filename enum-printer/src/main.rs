use windows::Win32::Graphics::Printing::{EnumPrintersW, PRINTER_ENUM_LOCAL, PRINTER_INFO_2W};

fn main() {
    let mut needed: u32 = 0;
    let mut returned: u32 = 0;

    unsafe {
        EnumPrintersW(
            PRINTER_ENUM_LOCAL,
            None,
            2,
            None,
            &mut needed,
            &mut returned,
        );
    }

    unsafe {
        let mut printer_enum = vec![0; needed as usize];

        EnumPrintersW(
            PRINTER_ENUM_LOCAL,
            None,
            2,
            Some(printer_enum.as_mut_slice()),
            &mut needed,
            &mut returned,
        );

        let arr: &[PRINTER_INFO_2W] = std::slice::from_raw_parts(
            printer_enum.as_ptr() as *mut PRINTER_INFO_2W,
            returned as usize,
        );

        for it in arr {
            let name = it.pPrinterName.to_string().unwrap();
            let driver = it.pDriverName.to_string().unwrap();
            println!("Name:{} Driver:{}", name, driver);
        }
    }
}

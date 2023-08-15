use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

fn main() {
    let status = unsafe {
        let mut status = MEMORYSTATUSEX {
            dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
            ..Default::default()
        };
        let _ = GlobalMemoryStatusEx(&mut status);

        status
    };

    println!("MemoryLoad: {}", status.dwMemoryLoad);
    println!("TotalPhys: {}", status.ullTotalPhys);
    println!("AvailPhys: {}", status.ullAvailPhys);
    println!("TotalPageFile: {}", status.ullTotalPageFile);
    println!("AvailPageFile: {}", status.ullAvailPageFile);
    println!("TotalVirtual: {}", status.ullTotalVirtual);
    println!("AvailVirtual: {}", status.ullAvailVirtual);
    println!("AvailExtendedVirtual: {}", status.ullAvailExtendedVirtual);
}

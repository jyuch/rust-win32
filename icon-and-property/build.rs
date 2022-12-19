use std::io;

fn main() -> io::Result<()> {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("media/icon.ico");
        res.compile()?;
    }

    Ok(())
}

use std::io;
use std::io::Write;
use std::ptr::write;
use qrcode_generator::{QRCodeError, QrCodeEcc};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let result = qrcode_generator::to_png_to_vec(buffer.as_str(), QrCodeEcc::High, 1024);

    match result {
        Ok(r) => {
            let mut lock = std::io::stdout().lock();

            lock.write_all(&r)?;

        }
        Err(e) => {eprint!("Error during qrcode generation! {}", e.to_string())}
    }
    Ok(())
}

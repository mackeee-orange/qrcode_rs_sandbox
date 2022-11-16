use anyhow::Result;
use qrcode::QrCode;
use image::Luma;
use std::env;

fn main() -> Result<()> {
    let val = &env::args().collect::<Vec<String>>()[1];
    let code = QrCode::new(val.as_bytes())?;
    let image = code.render::<Luma<u8>>().build();
    image.save("./qrcode.png")?;
    Ok(())
}

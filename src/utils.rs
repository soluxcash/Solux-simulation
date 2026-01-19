use qrcode::QrCode;
use image::Luma;

pub fn generate_qr_string(url: &str) -> String {
    // This returns a console-friendly QR code for simulation purposes
    let code = QrCode::new(url).unwrap();
    code.render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build()
}

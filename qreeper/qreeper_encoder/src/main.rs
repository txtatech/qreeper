extern crate image;
use gio::prelude::*;
use gtk::prelude::*;
use std::io::Write;
use qrcode::QrCode;
use gtk::{Window, WindowType, TextView, Box, Orientation, Button};
use image::DynamicImage;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use data_encoding::BASE64;
use image::Rgba;

fn main() {
    let application =
        gtk::Application::new(Some("com.example.qr_compressor"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&[]);
}

fn build_ui(application: &gtk::Application) {
    let window = Window::new(WindowType::Toplevel);
    window.set_application(Some(application));
    window.set_title("QR-DATA-COMPRESSOR");
    window.set_default_size(800, 600);

    let vbox = Box::new(Orientation::Vertical, 10);
    let text_input = TextView::new();
    text_input.set_wrap_mode(gtk::WrapMode::Word); // Ensure that the words wrap around nicely
    text_input.set_size_request(360, 240); // Set size to an estimated pixel size
    text_input.get_buffer().unwrap().set_text("Enter text");
    vbox.add(&text_input);


    let generate_qr_button = Button::with_label("Generate QR Codes");
    vbox.add(&generate_qr_button);

    let output_text = TextView::new();
    vbox.add(&output_text);

    window.add(&vbox);

    generate_qr_button.connect_clicked(move |_| {
        let text_input = text_input.clone();
        let output_text = output_text.clone();
        let buffer = text_input.get_buffer().unwrap();
        let start_iter = buffer.get_start_iter();
        let end_iter = buffer.get_end_iter();
        let qr_code_data = buffer.get_text(&start_iter, &end_iter, false).unwrap();
        let compressed_data_base64 = compress_data_base64(&qr_code_data);
        let compressed_data_hex = compress_data_hex(&qr_code_data);

        // Generate QR codes for both compressed data
        let qr_codes_base64 = generate_qr_codes(&compressed_data_base64).unwrap();
        for (i, qr_code) in qr_codes_base64.into_iter().enumerate() {
            let file_name = format!("compressed_qr_code_base64_{}.png", i + 1);
            qr_code.save(file_name).unwrap();
        }

        let qr_codes_hex = generate_qr_codes(&compressed_data_hex).unwrap();
        for (i, qr_code) in qr_codes_hex.into_iter().enumerate() {
            let file_name = format!("compressed_qr_code_hex_{}.png", i + 1);
            qr_code.save(file_name).unwrap();
        }

        // Show output
        let result_text = format!("Compressed (Base64): {}\nCompressed (Hex): {}", compressed_data_base64, compressed_data_hex);
        output_text.get_buffer().unwrap().set_text(&result_text);
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}

// Function to compress data and encode as Base64
fn compress_data_base64(data: &str) -> String {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes()).unwrap();
    let compressed_data = encoder.finish().unwrap();
    BASE64.encode(&compressed_data)
}

// Function to compress data and encode as Hex
fn compress_data_hex(data: &str) -> String {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes()).unwrap();
    let compressed_data = encoder.finish().unwrap();
    hex::encode(&compressed_data)
}

const QR_CHAR_LIMIT: usize = 4296;

fn generate_qr_codes(data: &str) -> Result<Vec<DynamicImage>, std::boxed::Box<dyn std::error::Error>> {
    let chunks: Vec<String> = data.chars()
        .collect::<Vec<_>>()
        .chunks(QR_CHAR_LIMIT)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect();

    let mut images = Vec::new();

    for chunk in chunks.into_iter() {
        let qr_code = QrCode::new(chunk)?.render::<Rgba<u8>>().build();
        let dynamic_image = DynamicImage::ImageRgba8(qr_code);

        images.push(dynamic_image);
    }

    Ok(images)
}

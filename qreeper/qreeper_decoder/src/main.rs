extern crate image;
use gio::prelude::*;
use gtk::prelude::*;
use std::io::Read;
use flate2::read::ZlibDecoder;
use data_encoding::BASE64;
use gtk::{Window, WindowType, TextView, Box, Orientation, Button, FileChooserButton, FileChooserAction};
extern crate rqrr;
use rqrr::PreparedImage;

fn main() {
    let application =
        gtk::Application::new(Some("com.example.qr_decompressor"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&[]);
}

fn build_ui(application: &gtk::Application) {
    let window = Window::new(WindowType::Toplevel);
    window.set_application(Some(application));
    window.set_title("QR-DATA-DECOMPRESSOR");
    window.set_default_size(800, 600);

    let vbox = Box::new(Orientation::Vertical, 10);

    let file_chooser = FileChooserButton::new("Select a File", FileChooserAction::Open);
    vbox.add(&file_chooser);

    let decode_button = Button::with_label("Decode QR Code");
    vbox.add(&decode_button);

    let output_text = TextView::new();
    vbox.add(&output_text);

    window.add(&vbox);

decode_button.connect_clicked(move |_| {
    match file_chooser.get_filename() {
        Some(file_path) => {
            let file_path_str = file_path.to_str().unwrap();
            let result_data = match decode_qr_code(file_path_str) {
                Ok(data) => data,
                Err(e) => format!("Error: {}", e),
            };
            output_text.get_buffer().unwrap().set_text(&result_data);
        },
        None => {
            output_text.get_buffer().unwrap().set_text("Please select a file first.");
        },
    }
});

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
}

fn decompress_data_base64(encoded_data: &str) -> Result<String, std::boxed::Box<dyn std::error::Error>> {
    // Decode the Base64 data
    let compressed_data = BASE64.decode(encoded_data.as_bytes())?;

    // Create a decoder and use it to decompress the data
    let mut decoder = ZlibDecoder::new(&compressed_data[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;

    // Convert the decompressed data to a string and return it
    Ok(String::from_utf8(decompressed_data)?)
}

fn decompress_data_hex(encoded_data: &str) -> Result<String, std::boxed::Box<dyn std::error::Error>> {
    // Decode the Hex data
    let compressed_data = hex::decode(encoded_data)?;

    // Create a decoder and use it to decompress the data
    let mut decoder = ZlibDecoder::new(&compressed_data[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;

    // Convert the decompressed data to a string and return it
    Ok(String::from_utf8(decompressed_data)?)
}


fn decode_qr_code(file_path: &str) -> Result<String, std::boxed::Box<dyn std::error::Error>> {
    // Open the image file
    let img = image::open(file_path)?.into_luma8();

    // Prepare the image
    let mut img = PreparedImage::prepare(img);

    // Search for grids, and decode the first one found
    let grid = img.detect_grids().pop().ok_or("No grids found")?;
    let (_meta, content) = grid.decode()?;

    // Try decompressing as Base64, then as Hex if that fails
    decompress_data_base64(&content).or_else(|_| decompress_data_hex(&content))
}

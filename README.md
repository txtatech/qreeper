# qreeper
A  qr code encoder and decoder that first uses gzip, on the data field text and then encodes a qr code with base64 and another qr code with hex.hex to generate qr codes

# QReeper

QReeper is a pair of Rust applications that provide QR code data compression and decompression functionality. 

The `qr_compressor` encodes and compresses given text data into QR codes in both Hex and Base64 encoding formats. 

The `qr_decompressor` is capable of decoding these QR codes and decompressing the data back into its original form.

The applications use a GTK interface.


## Installing QReeper

To install QReeper, follow these steps:

1. Clone the GitHub repository:

```bash
git clone https://github.com/txtatech/qreeper.git
```

2. Change into the `qreeper` directory:

```bash
cd qreeper
```

## Using QReeper

To use QReeper, you can run each program individually as follows:

### QReeper_Compressor

The QR Compressor encodes and compresses text data into QR codes. To use it, navigate to the `qr_compressor` directory and run the following command:

```bash
cargo run
```

This will launch the compressor application. You can enter your text data into the text field and click "Generate QR Codes" to create the QR codes. The compressed data will be shown in the application window and the generated QR codes will be saved as .png files in the same directory.

### QReeper_Decompressor

The QR Decompressor decodes and decompresses data from QR codes. To use it, navigate to the `qr_decompressor` directory and run the following command:

```bash
cargo run
```

This will launch the decompressor application. You can use the file chooser to select a QR code .png file and click "Decode QR Code" to decompress the data. The decompressed data will be shown in the application window.

## License

This project uses the [MIT License](https://opensource.org/licenses/MIT).

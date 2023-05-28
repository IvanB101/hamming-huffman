mod ext;

slint::include_modules!();

use ext::FfiError;
use rfd::FileDialog;

fn main() {
    let main_window = MainWindow::new().unwrap();

    main_window.on_choose_file(move |operation| {
        let valid_extentions: Vec<&str>;

        match operation.as_str() {
            "hamming" => {
                valid_extentions = ["txt"].into();
            }
            "dehamming" => {
                valid_extentions = ["HA1", "HA2", "HA3", "HE1", "HE2", "HE3"].into();
            }
            "corrupt" => {
                valid_extentions = ["HA1", "HA2", "HA3"].into();
            }
            "huffman" => {
                valid_extentions = ["txt", "doc", "docx"].into();
            }
            "dehuffman" => {
                valid_extentions = ["huf"].into();
            }
            _ => {
                valid_extentions = Vec::new();
            }
        }

        let path = match FileDialog::new()
            .add_filter("", valid_extentions.as_ref())
            .set_directory(".")
            .pick_file()
        {
            Some(path) => path,
            None => return,
        };

        let file_name = path.as_path().to_str().unwrap().to_string();

        let error = match operation.as_str() {
            "hamming" => ext::encode(file_name.to_owned(), 32),
            "dehamming" => ext::decode(file_name.to_owned(), true),
            "corrupt" => ext::corrupt(file_name.to_owned(), 0.50),
            "huffman" => ext::compress(file_name.to_owned()),
            "dehuffman" => ext::decompress(file_name.to_owned()),
            _ => Err(FfiError {
                message: "Invalid Operation".to_string(),
            }),
        };

        match error {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    });

    main_window.run().unwrap();
}

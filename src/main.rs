mod ext;

slint::include_modules!();

use ext::FfiError;
use rfd::FileDialog;

fn main() {
    let main_window = MainWindow::new().unwrap();
    let main_window_weak = main_window.as_weak();

    main_window
        .global::<State>()
        .on_operation(move |operation, value| {
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

            let path = match choose_file(valid_extentions) {
                Some(val) => val,
                None => return,
            };

            if let Err(e) = match operation.as_str() {
                "hamming" => {
                    let block_size = value.parse().unwrap();

                    ext::encode(path, block_size)
                }
                "dehamming" => {
                    let correct = value.parse().unwrap();

                    ext::decode(path, correct)
                }
                "corrupt" => ext::corrupt(path.to_owned(), 0.50),
                "huffman" => ext::compress(path.to_owned()),
                "dehuffman" => ext::decompress(path.to_owned()),
                _ => Err(FfiError {
                    message: "Invalid Operation".to_string(),
                }),
            } {
                main_window_weak.unwrap().set_error(e.to_string().into());
            }
        });

    main_window.run().unwrap();
}

fn choose_file(valid_extentions: Vec<&str>) -> Option<String> {
    Some(
        FileDialog::new()
            .add_filter("", valid_extentions.as_ref())
            .set_directory(".")
            .pick_file()?
            .as_path()
            .to_str()
            .unwrap()
            .to_string(),
    )
}

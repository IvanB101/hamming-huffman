mod ext;

slint::include_modules!();

use std::{fs::File, io::Read, rc::Rc};

use ext::Extention;
use rfd::FileDialog;
use slint::{Model, SharedString};

fn main() {
    let main_window = MainWindow::new().unwrap();

    let default_errors: Vec<SharedString> = main_window.get_errors().iter().collect();
    let errors = Rc::new(slint::VecModel::from(default_errors));
    main_window.set_errors(errors.clone().into());

    let default_in_progress: Vec<bool> = main_window.get_inProgress().iter().collect();
    let in_progress = Rc::new(slint::VecModel::from(default_in_progress));
    main_window.set_inProgress(in_progress.clone().into());

    let default_file_text: Vec<SharedString> = main_window.get_file_text().iter().collect();
    let file_text = Rc::new(slint::VecModel::from(default_file_text));
    main_window.set_file_text(file_text.clone().into());

    let errors_copy = errors.clone();
    main_window.global::<State>().on_protect(move |value| {
        let valid_extentions = ["txt"].into();

        let path = match choose_file(valid_extentions) {
            Some(val) => val,
            None => return,
        };

        let block_size = value.parse().unwrap();

        if let Err(e) = ext::encode(path, block_size) {
            errors_copy.set_row_data(0, e.to_string().into());
        }
    });

    let errors_copy = errors.clone();
    main_window.global::<State>().on_desprotect(move |correct| {
        let valid_extentions = ["HA1", "HA2", "HA3", "HE1", "HE2", "HE3"].into();

        let path = match choose_file(valid_extentions) {
            Some(val) => val,
            None => return,
        };

        if let Err(e) = ext::decode(path, correct) {
            errors_copy.set_row_data(1, e.to_string().into());
        }
    });

    let errors_copy = errors.clone();
    main_window
        .global::<State>()
        .on_corrupt(move |probability| {
            let valid_extentions = ["HA1", "HA2", "HA3"].into();

            let path = match choose_file(valid_extentions) {
                Some(val) => val,
                None => return,
            };

            if let Err(e) = ext::corrupt(path, probability.into()) {
                errors_copy.set_row_data(2, e.to_string().into());
            }
        });

    let errors_copy = errors.clone();
    main_window.global::<State>().on_compress(move || {
        let valid_extentions = ["txt", "doc", "docx"].into();

        let path = match choose_file(valid_extentions) {
            Some(val) => val,
            None => return,
        };

        if let Err(e) = ext::compress(path) {
            errors_copy.set_row_data(3, e.to_string().into());
        }
    });

    let errors_copy = errors.clone();
    main_window.global::<State>().on_decompress(move || {
        let valid_extentions = ["huf"].into();

        let path = match choose_file(valid_extentions) {
            Some(val) => val,
            None => return,
        };

        if let Err(e) = ext::decompress(path) {
            errors_copy.set_row_data(4, e.to_string().into());
        }
    });

    main_window.global::<State>().on_choose_file(move || {
        let valid_extentions = [
            "txt", "doc", "docx", "DE1", "DE2", "DE3", "DC1", "DC2", "DC3",
        ]
        .into();
        let error_extensions = ["HE1", "HE2", "HE3"];

        let path = match choose_file(valid_extentions) {
            Some(val) => val,
            None => return,
        };

        if let Some(_index) = error_extensions.iter().position(|&x| path.has_extention(x)) {
            // TODO Return Vec<String> with errors
        } else {
            let mut new_file_text: Vec<SharedString> = Vec::new();

            let mut contents = String::new();

            let mut file = File::open(path).unwrap();

            file.read_to_string(&mut contents).unwrap();

            new_file_text.push(contents.into());
            new_file_text.push("".into());
            new_file_text.push("".into());

            file_text.set_vec(new_file_text);
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

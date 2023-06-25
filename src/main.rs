mod buffered;
mod ext;

slint::include_modules!();

use std::{fs::File, io::Read, rc::Rc};

use ext::Extention;
use rfd::FileDialog;
use slint::{Model, SharedString, VecModel};

fn main() {
    let main_window = MainWindow::new().unwrap();

    let default_errors: Vec<SharedString> = main_window.get_errors().iter().collect();
    let errors = Rc::new(slint::VecModel::from(default_errors));
    main_window.set_errors(errors.clone().into());

    let default_in_progress: Vec<bool> = main_window.get_inProgress().iter().collect();
    let in_progress = Rc::new(slint::VecModel::from(default_in_progress));
    main_window.set_inProgress(in_progress.clone().into());

    let default_orig: Vec<SharedString> = main_window.get_orig_text().iter().collect();
    let orig_text = Rc::new(slint::VecModel::from(default_orig));
    main_window.set_orig_text(orig_text.clone().into());

    let default_proc: Vec<SharedString> = main_window.get_proc_text().iter().collect();
    let proc_text = Rc::new(slint::VecModel::from(default_proc));
    main_window.set_proc_text(proc_text.clone().into());

    let default_stat: Vec<SharedString> = main_window.get_stat().iter().collect();
    let stat = Rc::new(slint::VecModel::from(default_stat));
    main_window.set_stat(stat.clone().into());

    let default_hamming_stats: Vec<HammingStats> = main_window.get_hamming_stats().iter().collect();
    let hamming_stats = Rc::new(slint::VecModel::from(default_hamming_stats));
    main_window.set_hamming_stats(hamming_stats.clone().into());

    let default_huffman_stats: Vec<HuffmanStats> = main_window.get_huffman_stats().iter().collect();
    let huffman_stats = Rc::new(slint::VecModel::from(default_huffman_stats));
    main_window.set_huffman_stats(huffman_stats.clone().into());

    let errors_copy = errors.clone();
    main_window.global::<State>().on_protect(move |value| {
        handle_protect(value, errors_copy.clone());
    });

    let errors_copy = errors.clone();
    main_window.global::<State>().on_desprotect(move |correct| {
        handle_desprotect(correct, errors_copy.clone());
    });

    let errors_copy = errors.clone();
    main_window
        .global::<State>()
        .on_corrupt(move |prob1, prob2| {
            handle_corrupt(errors_copy.clone(), prob1, prob2);
        });

    let errors_copy = errors.clone();
    main_window.global::<State>().on_compress(move || {
        handle_compress(errors_copy.clone());
    });

    let errors_copy = errors.clone();
    main_window.global::<State>().on_decompress(move || {
        handle_decompress(errors_copy.clone());
    });

    let orig_copy = orig_text.clone();
    let stat_copy = stat.clone();
    main_window
        .global::<State>()
        .on_choose_file(move |operation| match operation.to_string().as_str() {
            "show" => handle_show_file(orig_copy.clone()),
            "stats" => handle_statistics(stat_copy.clone()),
            _ => return,
        });

    main_window.run().unwrap();
}

fn handle_protect(value: SharedString, errors: Rc<VecModel<SharedString>>) {
    let valid_extentions = ["txt"].into();

    let path = match choose_file(valid_extentions) {
        Some(val) => val,
        None => return,
    };

    let block_size = value.parse().unwrap();

    if let Err(e) = ext::encode(path, block_size) {
        errors.set_row_data(0, e.to_string().into());
    }
}

fn handle_desprotect(correct: bool, errors: Rc<VecModel<SharedString>>) {
    let valid_extentions = ["HA1", "HA2", "HA3", "HE1", "HE2", "HE3"].into();

    let path = match choose_file(valid_extentions) {
        Some(val) => val,
        None => return,
    };

    if let Err(e) = ext::decode(path, correct) {
        errors.set_row_data(1, e.to_string().into());
    }
}

fn handle_corrupt(errors: Rc<VecModel<SharedString>>, prob1: f32, prob2: f32) {
    let valid_extentions = ["HA1", "HA2", "HA3"].into();

    let path = match choose_file(valid_extentions) {
        Some(val) => val,
        None => return,
    };

    if let Err(e) = ext::corrupt(path, prob1.into()) {
        errors.set_row_data(2, e.to_string().into());
    }
}

fn handle_compress(errors: Rc<VecModel<SharedString>>) {
    let valid_extentions = ["txt", "doc", "docx"].into();

    let path = match choose_file(valid_extentions) {
        Some(val) => val,
        None => return,
    };

    if let Err(e) = ext::compress(path) {
        errors.set_row_data(3, e.to_string().into());
    }
}

fn handle_decompress(errors: Rc<VecModel<SharedString>>) {
    let valid_extentions = ["huf"].into();

    let path = match choose_file(valid_extentions) {
        Some(val) => val,
        None => return,
    };

    if let Err(e) = ext::decompress(path) {
        errors.set_row_data(4, e.to_string().into());
    }
}

fn handle_show_file(orig_text: Rc<VecModel<SharedString>>) {
    let valid_extentions = [
        "txt", "doc", "docx", "DE1", "DE2", "DE3", "DC1", "DC2", "DC3",
    ]
    .into();
    let error_extensions = ["HE1", "HE2", "HE3"];

    let path = match choose_file(valid_extentions) {
        Some(val) => val,
        None => return,
    };

    let mut file = File::open(&path).unwrap();

    if let Some(_index) = error_extensions.iter().position(|&x| path.has_extention(x)) {
        // TODO Return Vec<String> with errors
    } else {
        let mut new_file_text: Vec<SharedString> = Vec::new();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        new_file_text.push(contents.into());
        new_file_text.push("Err1".into());

        orig_text.set_vec(new_file_text);
    }
}

fn handle_statistics(stat: Rc<VecModel<SharedString>>) {
    let valid_extentions = ["HA1", "HA2", "HA3", "HE1", "HE2", "HE3", "huf"].into();
    let hamming_extentions: Vec<&str> = ["HA1", "HA2", "HA3", "HE1", "HE2", "HE3"].into();

    let path = match choose_file(valid_extentions) {
        Some(val) => val,
        None => return,
    };

    let mut new_stat: Vec<SharedString> = Vec::new();
    if let Some(_) = hamming_extentions
        .iter()
        .position(|&x| path.has_extention(x))
    {
        new_stat.push("hamming".into());
        stat.set_vec(new_stat);
    } else {
        new_stat.push("huffman".into());
        stat.set_vec(new_stat);
    }
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

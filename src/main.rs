mod buffered;
mod ext;
mod hamming;
mod util;

slint::include_modules!();

use buffered::reader::{read_f64, read_u32, read_u64, read_u8};
use ext::Extention;
use hamming::{init_masks, MAX_BLOCK_SIZE, MAX_EXPONENT};
use rfd::FileDialog;
use slint::{Model, SharedString, VecModel};
use std::{
    fs::File,
    io::{BufReader, Read},
    rc::Rc,
};

fn main() {
    let masks = init_masks();
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

    let default_huffman_table: Vec<HuffmanEntry> = main_window.get_huffman_table().iter().collect();
    let huffman_table = Rc::new(slint::VecModel::from(default_huffman_table));
    main_window.set_huffman_table(huffman_table.clone().into());

    let errors_copy = errors.clone();
    main_window.global::<State>().on_protect(move |value| {
        handle_protect(value, errors_copy.clone(), &masks);
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
    let huffman_copy = huffman_stats.clone();
    let hamming_copy = hamming_stats.clone();
    let table_copy = huffman_table.clone();
    main_window
        .global::<State>()
        .on_choose_file(move |operation| match operation.to_string().as_str() {
            "show" => handle_show_file(orig_copy.clone()),
            "stats" => {
                if let Err(e) = handle_statistics(
                    stat_copy.clone(),
                    hamming_copy.clone(),
                    huffman_copy.clone(),
                    table_copy.clone(),
                ) {
                    // TODO handling
                    println!("{}", e);
                }
            }
            _ => return,
        });

    main_window.run().unwrap();
}

fn handle_protect(
    value: SharedString,
    errors: Rc<VecModel<SharedString>>,
    masks: &[[u8; MAX_BLOCK_SIZE]; MAX_EXPONENT],
) {
    let valid_extentions = ["txt"].into();

    let path = match choose_file(valid_extentions) {
        Some(val) => val,
        None => return,
    };

    let block_size = value.parse().unwrap();

    if let Err(e) = hamming::encoder::encode(&path, block_size as usize, &masks) {
        println!("Error {}", e);
    }

    return;
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

fn handle_statistics(
    stat: Rc<VecModel<SharedString>>,
    hamming_stats: Rc<VecModel<HammingStats>>,
    huffman_stats: Rc<VecModel<HuffmanStats>>,
    huffman_table: Rc<VecModel<HuffmanEntry>>,
) -> Result<(), std::io::Error> {
    let block_sizes = [32, 2048, 65526];
    let exponents = [5, 11, 16];
    let valid_extentions = ["HA1", "HA2", "HA3", "HE1", "HE2", "HE3", "huf"].into();
    let hamming_extentions: Vec<&str> = ["HA1", "HA2", "HA3", "HE1", "HE2", "HE3"].into();

    let path = match choose_file(valid_extentions) {
        Some(val) => val,
        None => return Ok(()),
    };

    let mut new_stat: Vec<SharedString> = Vec::new();
    if let Some(index) = hamming_extentions
        .iter()
        .position(|&x| path.has_extention(x))
    {
        new_stat.push("hamming".into());
        stat.set_vec(new_stat);
        let block_size = block_sizes[index % 3];
        let exponent = exponents[index % 3];

        let mut new_hamming_stats: Vec<HammingStats> = Vec::new();

        let mut file_size: u64 = 0;
        let mut n_blocks: u64 = 0;
        let mut reader = BufReader::new(File::open(path)?);

        read_u64(&mut reader, &mut n_blocks)?;
        read_u64(&mut reader, &mut file_size)?;

        let info_bits = file_size * 8;
        let protection_bits = n_blocks * exponent;
        let filler_bits = n_blocks * block_size - info_bits - protection_bits;

        new_hamming_stats.push(HammingStats {
            filler_bits: filler_bits as i32,
            info_bits: info_bits as i32,
            protection_bits: protection_bits as i32,
        });

        hamming_stats.set_vec(new_hamming_stats);
    } else {
        new_stat.push("huffman".into());
        stat.set_vec(new_stat);

        let mut new_huffman_stats: Vec<HuffmanStats> = Vec::new();
        let mut new_huffman_table: Vec<HuffmanEntry> = Vec::new();

        let fd = File::open(path)?;
        let comp_size = fd.metadata()?.len();
        let mut reader = BufReader::new(fd);
        let mut distinc: u32 = 0;
        let mut orig_size: u64 = 0;

        read_u32(&mut reader, &mut distinc)?;
        println!("Distinct: {}", distinc);

        for _i in 0..distinc {
            let mut original: u8 = 0;
            let mut length: u8 = 0;
            let compressed: String = "hola".into();
            let mut prob: f64 = 0.0;

            original = read_u8(&mut reader)?;
            length = read_u8(&mut reader)?;
            let mut buffer: Vec<u8> = Vec::with_capacity(length.into());
            reader.read_exact(&mut buffer)?;
            read_f64(&mut reader, &mut prob)?;

            println!("Length: {}", length);

            new_huffman_table.push(HuffmanEntry {
                comp: compressed.into(),
                orig: format!("{} {original:b}", original as char).into(),
                prob: prob as f32,
            });
        }

        read_u64(&mut reader, &mut orig_size)?;

        new_huffman_stats.push(HuffmanStats {
            comp_size: comp_size as i32,
            orig_size: orig_size as i32,
        });

        huffman_stats.set_vec(new_huffman_stats);
        huffman_table.set_vec(new_huffman_table);
    }

    Ok(())
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

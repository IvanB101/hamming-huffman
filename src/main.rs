mod ext;

use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() < 1 {
        print_usage();
    }

    let mut err = Ok(());

    match args[0].as_str() {
        "ham" => {
            if args.len() < 3 {
                print_usage()
            }
            match args[2].parse::<u64>() {
                Ok(number) => err = ext::encode(args[1].to_owned(), number),
                Err(_) => print_usage(),
            }
        }
        "deham" => {
            if args.len() < 3 {
                print_usage()
            }
            match args[2].parse::<u64>() {
                Ok(correct) => err = ext::decode(args[1].to_owned(), correct != 0),
                Err(_) => print_usage(),
            }
        }
        "huf" => {
            if args.len() < 2 {
                print_usage()
            }
            err = ext::compress(args[1].to_owned());
        }
        "dehuf" => {
            if args.len() < 2 {
                print_usage()
            }
            err = ext::decompress(args[1].to_owned());
        }
        "cor" => {
            if args.len() < 3 {
                print_usage()
            }
            match args[2].parse::<f64>() {
                Ok(prob) => err = ext::corrupt(args[1].to_owned(), prob),
                Err(_) => print_usage(),
            }
        }
        _ => print_usage(),
    }

    match err {
        Ok(_) => println!("Operacion completada exitosamente"),
        Err(err) => println!("Error: {}", err),
    }
}

fn print_usage() {
    println!("Uso");
    exit(1);
}

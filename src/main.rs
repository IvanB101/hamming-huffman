mod ext;

use ext::Extention;

fn main() {
    let path = String::from("test/Primero.txt");
    /*
    print!("Encode: ");
    match ext::encode(path.clone(), 32) {
        Ok(()) => println!("Success"),
        Err(err) => println!("{}", err.message),
    };

    print!("Corrupt: ");
    match ext::corrupt(path.clone().with_extention("HA1")) {
        Ok(()) => println!("Success"),
        Err(err) => println!("{}", err.message),
    };

    print!("Decode: ");
    match ext::decode(path.clone().with_extention("HE1"), false) {
        Ok(()) => println!("Success"),
        Err(err) => println!("{}", err.message),
    };
    */

    print!("Compress: ");
    match ext::compress(path.clone().with_extention("txt")) {
        Ok(()) => println!("Success"),
        Err(err) => println!("{}", err.message),
    };

    print!("Decompress: ");
    match ext::decompress(path.clone().with_extention("huf")) {
        Ok(()) => println!("Success"),
        Err(err) => println!("{}", err.message),
    };
}

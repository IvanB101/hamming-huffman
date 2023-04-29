mod ext;

fn main() {
    let path = String::from("Primero.txt");
    match ext::encode(path, 32) {
        Ok(()) => println!("Succes"),
        Err(err) => println!("{}", err.message),
    }
}

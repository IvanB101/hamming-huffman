mod ext;

fn main() {
    ext::encode(String::from("Primero.txt"), 32).unwrap();
}

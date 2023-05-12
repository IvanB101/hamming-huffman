fn main() {
    println!(r"cargo:rustc-link-search=src/ext/build");
    println!(r"cargo:rustc-link-lib=static=c_procedures");
}

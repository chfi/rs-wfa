fn main() {
    println!("cargo:rustc-link-search=native=libwfa");
    println!("cargo:rustc-link-lib=static=wfa");
}

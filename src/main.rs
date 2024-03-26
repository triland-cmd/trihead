fn main() {
    if let Err(error) = trihead::run() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        if std::path::Path::new("assets/icon.ico").exists() {
            let mut res = winres::WindowsResource::new();
            res.set_icon("assets/icon.ico");
            res.compile().unwrap();
        } else {
            println!("cargo:warning=Icon file not found at assets/icon.ico. Skipping icon embedding.");
        }
    }
}

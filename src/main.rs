use std::env;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;

fn main() {
    let exe_path = env::current_exe().expect("Failed to get the executable path");
    let exe_dir = exe_path
        .parent()
        .expect("Failed to get the executable directory");

    let url = "https://example.com/path/to/your/file.zip";
    let response = reqwest::blocking::get(url).expect("Failed to download the file");
    let mut file = File::create(exe_dir.join("file.zip")).expect("Failed to create file");
    let mut content = std::io::Cursor::new(response.bytes().expect("Failed to read content"));

    copy(&mut content, &mut file).expect("Failed to copy content to file");

    println!("Downloaded zip file to {:?}", exe_dir.join("file.zip"));
}

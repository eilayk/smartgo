mod gtfs_to_db;

use std::path::Path;
use std::fs;
use reqwest::blocking::get;
use bytes::Bytes;
use zip::read::ZipArchive;
use std::io::Write;

pub fn initialize_content_directories() {
    let content_path = Path::new("./content/");
    if !content_path.exists() {
        fs::create_dir(content_path).expect("file system error: failed to create content directory");
    }
}

pub fn get_zip_byte_content() -> Bytes {
    let target = "https://assets.metrolinx.com/raw/upload/Documents/Metrolinx/Open%20Data/GO-GTFS.zip";
    let response = get(target).expect("failed to retrieve zip file");
    let bytes = response.bytes().expect("failed to retrieve zip bytes");
    bytes
}

// pub fn create_zip_from_bytes(bytes: Bytes) -> fs::File{
//     let path = Path::new("./content/gtfs.zip");
//     let mut file = fs::File::create(path).expect("file system error: failed to create zip file in local machine");
//     file.write(&bytes).expect("file system error: failed to initialize zip file");
//     file
// }

pub fn create_zip_from_bytes() -> fs::File{
    let path = Path::new("./content/gtfs.zip");
    let file = fs::File::open(path).expect("file system error: failed to create zip file in local machine");
    file
}

pub fn gen_db_from_zip(zip_file: fs::File) {
    // open zip archive
    let zip_archive = ZipArchive::new(zip_file).expect("Failed to unzip zip file");
    gtfs_to_db::gen_db(zip_archive);
}

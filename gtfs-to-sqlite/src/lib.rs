mod gtfs_to_db;

use std::io::Write;
use std::path::Path;
use std::fs;
use chrono::NaiveDate;
use reqwest::blocking::get;
use bytes::Bytes;
use zip::read::ZipArchive;

pub fn initialize_content_directories() {
    let content_path = Path::new("./content/");
    if !content_path.exists() {
        fs::create_dir(content_path).expect("file system error: failed to create content directory");
    }
}

pub fn get_zip_byte_content_from_go() -> Bytes {
    let target = "https://assets.metrolinx.com/raw/upload/Documents/Metrolinx/Open%20Data/GO-GTFS.zip";
    let response = get(target).expect("failed to retrieve zip file");
    let bytes = response.bytes().expect("failed to retrieve zip bytes");
    bytes
}

pub fn create_zip_from_bytes(bytes: Bytes) -> fs::File{
    let path = Path::new("./content/gtfs.zip");
    if path.exists() {
        fs::remove_file(path).expect("file system error: failed to remove existing zip file");
    }
    let mut file = fs::File::create(path).expect("file system error: failed to create zip file in local machine");
    file.write(&bytes).expect("file system error: failed to initialize zip file");
    file
}

pub fn open_zip() -> fs::File{
    let path = Path::new("./content/gtfs.zip");
    let file = fs::File::open(path).expect("file system error: failed to create zip file in local machine");
    file
}

pub fn gen_db_from_zip(zip_file: fs::File) {
    // open zip archive
    let zip_archive = ZipArchive::new(zip_file).expect("Failed to unzip zip file");
    gtfs_to_db::gen_db(zip_archive);
}

pub fn gtfs_recently_fetched() -> bool {
    let path = Path::new("./content/gtfs.zip");
    if !path.exists() {
        return false;
    }
    let metadata = fs::metadata(path).expect("file system error: failed to get metadata of zip file");
    let elapsed_seconds = metadata.created().unwrap().elapsed().unwrap().as_secs();
    if elapsed_seconds > 60 * 60 * 24 {
        // file was fetched more than a day ago
        return false;
    }
    true
}

fn parse_mmyy(mmyy: &str) -> Option<NaiveDate> {
    // Ensure the input string is exactly 4 characters long
    if mmyy.len() != 4 {
        return None;
    }

    // Split the string into month and year parts
    let month = mmyy[0..2].parse::<u32>().ok()?;
    let year = mmyy[2..4].parse::<i32>().ok()?;

    // Convert the 2-digit year to a 4-digit year (assuming 2000s)
    let year = 2000 + year;

    // Try to create a NaiveDate
    NaiveDate::from_ymd_opt(year, month, 1)
}
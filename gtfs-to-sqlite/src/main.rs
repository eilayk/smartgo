use gtfs::{initialize_content_directories, get_zip_byte_content, create_zip_from_bytes, gen_db_from_zip};


// fn main() {
//     let target = "https://assets.metrolinx.com/raw/upload/Documents/Metrolinx/Open%20Data/GO-GTFS.zip";
//     let path = Path::new("./download/GTFS.zip");

//     let resp = reqwest::blocking::get(target).unwrap();
//     let mut file = File::create(path).unwrap();
//     let content = resp.bytes().unwrap();
//     file.write(&content);
//     // std::io:copy(&mut content, &mut file);
// }

fn main() {
    initialize_content_directories();

    // get from go
    // let bytes = get_zip_byte_content();
    // let zip = create_zip_from_bytes(bytes);
    // local testing
    let zip = create_zip_from_bytes();
    
    gen_db_from_zip(zip);
}
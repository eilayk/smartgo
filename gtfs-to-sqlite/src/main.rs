use std::fs::File;

use gtfs::{create_zip_from_bytes, gen_db_from_zip, get_zip_byte_content_from_go, gtfs_recently_fetched, initialize_content_directories, open_zip};

fn main() {
    initialize_content_directories();
    let zip: File;
    // get from go
    if !gtfs_recently_fetched() {
        let bytes = get_zip_byte_content_from_go();
        zip = create_zip_from_bytes(bytes);
    } else {
        zip = open_zip();
    }

    gen_db_from_zip(zip);
}
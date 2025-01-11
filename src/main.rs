use std::io::Read;

use bio_read::BioReader;

fn main() {
    let reader = BioReader::new();
    // Read from stdin until EOF
    let mut text = String::new();
    std::io::stdin().read_to_string(&mut text).expect("Failed to read from stdin");
    let bio_read_text = reader.bio_read_text(&text);
    println!("{}", bio_read_text);
}

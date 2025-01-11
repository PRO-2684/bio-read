use std::io::Read;

use bio_read::BioReader;

fn main() {
    let reader = BioReader::new();
    // let text = "An apple a day keeps the doctor away.";
    // Read from stdin until EOF
    let mut text = String::new();
    std::io::stdin().read_to_string(&mut text).unwrap();
    let bio_read_text = reader.bio_read_text(&text);
    println!("{}", bio_read_text);
}

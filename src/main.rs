use argh::FromArgs;
use bio_read::BioReader;
use std::io::Read;

#[derive(FromArgs)]
/// Bionic reading in terminal.
pub struct Args {
    /// the fixation point. Should be in range [1, 5]. Default is 3.
    #[argh(option, short = 'f', default = "3")]
    fixation_point: usize,
}

fn main() {
    let args: Args = argh::from_env();
    let fixation_point = args.fixation_point;
    if fixation_point < 1 || fixation_point > 5 {
        eprintln!("Fixation point should be in range [1, 5], but got {}", fixation_point);
        std::process::exit(1);
    }
    let reader = BioReader::new().fixation_point(fixation_point);
    // Read from stdin until EOF
    let mut text = String::new();
    std::io::stdin()
        .read_to_string(&mut text)
        .expect("Failed to read from stdin");
    let bio_read_text = reader.bio_read_text(&text);
    println!("{}", bio_read_text);
}

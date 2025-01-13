use argh::FromArgs;
use bio_read::BioReader;
use std::io::{Lines, BufRead, BufReader};

#[derive(FromArgs)]
/// Bionic reading in terminal.
pub struct Args {
    /// the fixation point. Should be in range [1, 5]. Default is 3.
    #[argh(option, short = 'f', default = "3")]
    fixation_point: usize,
    /// the file to read from. Read from stdin if not specified.
    #[argh(option, short = 'i')]
    input: Option<String>,
}

fn process_lines(reader: &BioReader, lines: Lines<impl BufRead>) {
    for line in lines {
        let line = line.expect("Failed to read line");
        let bio_read_text = reader.bio_read_text(&line);
        println!("{}", bio_read_text);
    }
}

fn main() {
    let args: Args = argh::from_env();
    let fixation_point = args.fixation_point;
    if fixation_point < 1 || fixation_point > 5 {
        eprintln!("Fixation point should be in range [1, 5], but got {}", fixation_point);
        std::process::exit(1);
    }
    let reader = BioReader::new().fixation_point(fixation_point);
    match args.input {
        Some(path) => {
            // Read from file
            let file = std::fs::File::open(path).expect("Failed to open file");
            let buffer = BufReader::new(file);
            process_lines(&reader, buffer.lines());
        },
        None => {
            // Read from stdin
            process_lines(&reader, std::io::stdin().lock().lines());
        },
    }
}

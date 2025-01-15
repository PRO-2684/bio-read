use argh::FromArgs;
use bio_read::BioReader;

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

fn main() -> std::io::Result<()> {
    let args: Args = argh::from_env();
    let fixation_point = args.fixation_point;
    if fixation_point < 1 || fixation_point > 5 {
        eprintln!("Fixation point should be in range [1, 5], but got {}", fixation_point);
        std::process::exit(1);
    }
    let reader = BioReader::new()
        .fixation_point(fixation_point);
    let mut lock = std::io::stdout().lock();
    match args.input {
        Some(path) => {
            // Read from file
            let file = std::fs::File::open(path)?;
            reader.bio_read(file, &mut lock)?;
        },
        None => {
            // Read from stdin
            reader.bio_read(std::io::stdin().lock(), &mut lock)?;
        },
    }
    Ok(())
}

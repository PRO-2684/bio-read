use argh::FromArgs;
use bio_read::BioReader;

#[derive(FromArgs)]
/// Bionic reading in terminal.
#[argh(help_triggers("-h", "--help"))]
pub struct Args {
    /// the file to read from. Read from stdin if not specified.
    #[argh(positional)]
    input: Option<String>,
    /// the fixation point. Should be in range [1, 5]. Default is 3.
    #[argh(option, short = 'f', default = "3")]
    fixation_point: usize,
    /// customize how to emphasize the text. The emphasized text will take the place of "{}". Example: --emphasize "<em>{}</em>". Default to ansi bold.
    #[argh(option, short = 'e')]
    emphasize: Option<String>,
    /// customize how to de-emphasize the text. The de-emphasized text will take the place of "{}". Example: --de-emphasize "<de>{}</de>". Default to ansi dimmed.
    #[argh(option, short = 'd')]
    de_emphasize: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args: Args = argh::from_env();
    let fixation_point = args.fixation_point;
    if fixation_point < 1 || fixation_point > 5 {
        eprintln!(
            "Fixation point should be in range [1, 5], but got {}",
            fixation_point
        );
        std::process::exit(1);
    }
    let mut reader = BioReader::new().fixation_point(fixation_point);
    if let Some(emphasize) = args.emphasize {
        let (left, right) = emphasize.split_once("{}").unwrap_or_else(|| {
            eprintln!("Invalid emphasize format: {}", emphasize);
            std::process::exit(1);
        });
        reader = reader.emphasize(left.to_string(), right.to_string());
    }
    if let Some(de_emphasize) = args.de_emphasize {
        let (left, right) = de_emphasize.split_once("{}").unwrap_or_else(|| {
            eprintln!("Invalid de-emphasize format: {}", de_emphasize);
            std::process::exit(1);
        });
        reader = reader.de_emphasize(left.to_string(), right.to_string());
    }
    let mut lock = std::io::stdout().lock();
    match args.input {
        Some(path) => {
            // Read from file
            let file = std::fs::File::open(path)?;
            reader.bio_read(file, &mut lock)?;
        }
        None => {
            // Read from stdin
            reader.bio_read(std::io::stdin().lock(), &mut lock)?;
        }
    }
    Ok(())
}

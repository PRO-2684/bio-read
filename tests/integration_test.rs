use bio_read::BioReader;
use std::{fs, path::Path};

fn setup_reader(fixation_point: usize) -> BioReader {
    let reader = BioReader::new()
        .emphasize("<em>".to_string(), "</em>".to_string())
        .de_emphasize("<de>".to_string(), "</de>".to_string())
        .fixation_point(fixation_point);
    reader
}

#[test]
fn test_bio_read_simple() {
    let reader = setup_reader(3);
    assert_eq!(reader.bio_read_text("hello world"), "<em>hel</em><de>lo</de> <em>wor</em><de>ld</de>");
}

#[test]
fn test_bio_read_on_files() {
    let reader = setup_reader(3);
    // tests/input/* -> tests/output/*
    let files = fs::read_dir("tests/input").unwrap();
    for file in files {
        let file = file.unwrap();
        let path = file.path();
        let text = fs::read_to_string(&path).unwrap();
        let output = reader.bio_read_text(&text);
        let output_path = Path::new("tests/output").join(path.file_name().unwrap());
        let expected_output = fs::read_to_string(&output_path).unwrap();
        assert_eq!(output, expected_output);
    }
}

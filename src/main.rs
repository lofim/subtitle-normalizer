extern crate unicode_normalization;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use unicode_normalization::UnicodeNormalization;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        panic!("Wrong number of parameter. Expected file name.");
    }

    let file_path = Path::new(&args[1]);
    let file_name = file_path
        .file_name()
        .expect("File path has to contain file name");

    let mut file = File::open(file_path).expect("Unable to open the input file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    let normalized = contents.nfd().filter(|c| c.is_ascii()).collect::<String>();

    let mut output_file_name = file_name.to_owned();
    output_file_name.push(".normalized");

    let output_path = file_path.with_file_name(output_file_name);
    let mut file = File::create(output_path).expect("Unable to create the output file");

    file.write_all(normalized.as_bytes())
        .expect("Unable to write the output file");
}

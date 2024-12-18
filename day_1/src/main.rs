use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let input_file: &str = "day_1_input.txt";
    let data_string: String = file_to_string(input_file);
    print!("{} conttains:\n{}\n", input_file, data_string);
}

fn file_to_string(filename: &str) -> String {
    // Create a path to the desired file
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(s) => s,
    };

    return s;

    // `file` goes out of scope, and the "hello.txt" file gets closed
}
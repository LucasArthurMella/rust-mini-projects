use crate::args::Args;
use std::fs;

pub fn handle_cat(args: Args) {
    let values = args.values.expect("No file paths provided!");

    let mut concatenated_text = String::from("");

    for file_path in values.iter() {
        let file_contents = fs::read_to_string(file_path).expect("There was a problem opening this file!");
        concatenated_text.push_str(file_contents.as_str());
    } 

    println!("{}", concatenated_text);
}

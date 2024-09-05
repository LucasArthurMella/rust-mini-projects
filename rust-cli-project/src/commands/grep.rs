use crate::args::Args;
use std::fs::read_to_string;

pub fn handle_grep(args: Args){
    let values = args.values.expect("No arguments provided!");
    let file_path = values.get(0).expect("No filepath provided!");
    let grep_text = values.get(1).expect("No grep text provided!");
    let contents = read_to_string(file_path).expect("Failed to read the file!");

    for line in contents.lines(){
        if line.contains(grep_text){
            println!("{}",line);    
        }
    }

}

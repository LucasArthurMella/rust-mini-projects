use crate::args::Args;
use std::fs;
use std::path::Path;

pub fn handle_find(args: Args){
    let values = args.values.expect("No arguments provided!");
    let provided_file_name = values.get(0).expect("No filename provided!");
    let root = Path::new(".");
    let mut dirs_to_check = vec![root.to_path_buf()];

    while let Some(current_dir) = dirs_to_check.pop() {
        if let Ok(entries) = fs::read_dir(&current_dir){
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if let Some(filename) = path.file_name(){
                        if let Some(i) = filename.to_str() {
                            if i.contains(provided_file_name){
                                if let Some(path_string) = path.to_str(){
                                    println!("{}", path_string);
                                }
                            }
                        } 
                    }

                    if path.is_dir() {
                        dirs_to_check.push(path);
                    }
                }
            }    
        }
    }
}






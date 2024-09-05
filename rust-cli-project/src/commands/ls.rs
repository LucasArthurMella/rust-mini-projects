use std::fs;

pub fn handle_ls() {
    let directories = fs::read_dir("./").expect("Failed at reading directories!");
    
    for directory in directories {
        println!("{} ", directory.expect("Failed at acquiring directory!").path().display());
    }
}

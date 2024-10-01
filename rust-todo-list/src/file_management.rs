use std::fs::{OpenOptions,File};
use std::io::BufReader;
use crate::todo_operations::Todo;

pub fn serialize_file() -> Vec<Todo> {
    let opened_file = File::open("todo_list.txt");
    match opened_file {
        Ok(i) => {
            let reader = BufReader::new(i);
            let list_contents: Vec<Todo> = serde_json::from_reader(reader).expect("Failed to read todo list contents!");
            return list_contents; 
        },
        Err(_) => {
            let empty_todo_list: Vec<Todo> = vec!();
            write_todo_list(&empty_todo_list);
            return empty_todo_list;
        }
    };
}

pub fn write_todo_list(list: &Vec<Todo>){
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .read(true)
        .open("todo_list.txt").expect("Failed to create todo file!");
    let _ = serde_json::to_writer(&mut file, list);
}

use crate::todo_operations::TodoOperation;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    command: String,
    pub values: Option<Vec<String>>
}

pub fn get_args_operation() -> TodoOperation {
    let args = Args::parse(); 
    match args.command.as_str() {
        "list" => TodoOperation::List(args),
        "add" => TodoOperation::Add(args),
        "done" => TodoOperation::Done(args),
        "edit" => TodoOperation::Edit(args),
        "remove" => TodoOperation::Remove(args),
        _ => panic!("Command not found!")
    }
}

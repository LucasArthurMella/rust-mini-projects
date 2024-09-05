use crate::commands::Command;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    command: String,
    pub values: Option<Vec<String>>
}

pub fn get_args_command() -> Command{
    let args = Args::parse();
    match args.command.as_str() {
        "echo" => Command::Echo(args),
        "cat" => Command::Cat(args),
        "ls" => Command::Ls(args),
        "find" => Command::Find(args),
        "grep" => Command::Grep(args),
        _ => panic!("Command not found!")
    }
}

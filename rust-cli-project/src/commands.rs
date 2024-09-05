use crate::args::Args;
use echo::handle_echo;
use cat::handle_cat;
use ls::handle_ls;
use find::handle_find;
use grep::handle_grep;

pub mod cat;
pub mod find;
pub mod grep;
pub mod ls;
pub mod echo;

pub enum Command {
    Cat(Args),
    Find(Args),
    Grep(Args),
    Ls(Args),
    Echo(Args)
}

pub fn handle_command(command: Command){
    match command {
        Command::Echo(i) => handle_echo(i),
        Command::Cat(i) => handle_cat(i),
        Command::Ls(_) => handle_ls(),
        Command::Find(i) => handle_find(i), 
        Command::Grep(i) => handle_grep(i) 
    }
}

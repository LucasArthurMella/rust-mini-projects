use rust_cli::args::get_args_command;
use rust_cli::commands::handle_command;

fn main() {
    let command = get_args_command();
    handle_command(command);
}

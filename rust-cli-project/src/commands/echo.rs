use crate::args::Args;


pub fn handle_echo(args: Args) {
    if let Some(i) = args.values {
        println!("{}", i[0]);
    }
}

use rust_todo_list::args::get_args_operation;
use rust_todo_list::todo_operations::TodoOperation;
use rust_todo_list::file_management::serialize_file;

fn main() {
    let todo_list = serialize_file();
    let operation = get_args_operation();
    match operation {
        TodoOperation::List(_args) => {
            TodoOperation::list_todos(todo_list);
        },
        TodoOperation::Add(args) => {
            let todos_to_add = args.values.expect("Todos to add shouldn't be empty!"); 
            TodoOperation::add_todo(todo_list, todos_to_add);
        },
        TodoOperation::Done(args) => {
            let todos_to_done:Vec<usize> = args.values.expect("Todos to todo shouldn't be empty!")
                .into_iter()
                .map(|todo_to_done| {
                    let int_todo_to_done:usize = todo_to_done.parse().expect("Todos to todo should all be integers!");
                    return int_todo_to_done;
                })
                .collect();
            TodoOperation::done_todo(todo_list, todos_to_done); 
        },
        TodoOperation::Edit(args) => {
            let args = args.values.expect("Args shouldn't be empty!"); 
            let index: usize = args.get(0).expect("Index should not be empty!").parse().expect("Index should be a positive number!"); 
            let new_todo_name = args.get(1).expect("New todo name shouldn't be empty!");
            TodoOperation::edit_todo(todo_list, index, new_todo_name.to_string());
        },
        TodoOperation::Remove(args) => {
            let args = args.values.expect("Args shouldn't be empty!"); 
            let index: usize = args.get(0).expect("Index should not be empty!").parse().expect("Index should be a positive number!"); 
            TodoOperation::remove_todo(todo_list, index);
        }
    }
}



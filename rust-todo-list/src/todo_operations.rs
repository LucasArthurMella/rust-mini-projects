use crate::args::Args;
use crate::file_management::write_todo_list;
use serde::{Serialize, Deserialize};

pub enum TodoOperation {
    List(Args),
    Add(Args),
    Done(Args),
    Edit(Args),
    Remove(Args)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    name: String,
    done: bool
}

impl TodoOperation {
    pub fn list_todos(current_todos:Vec<Todo>){
        for (i, el) in current_todos.iter().enumerate(){
            if el.done {
                println!("(X){}-{}", i+1, el.name);
            }else{
                println!("( ){}-{}", i+1, el.name);
            }
        }
    }

    pub fn add_todo(current_todos:Vec<Todo>, add_todos: Vec<String>){
        let mut new_todos = current_todos;
        let add_todos: Vec<Todo> = add_todos
            .into_iter()
            .map(|name| Todo {name, done: false })
            .collect();

        new_todos.extend(add_todos);
        write_todo_list(&new_todos);
        Self::list_todos(new_todos);
    }

    pub fn done_todo(mut current_todos: Vec<Todo>, todo_indexes: Vec<usize>){
        todo_indexes
            .into_iter()
            .for_each(|index| {
                match current_todos.get_mut(index-1) {
                    Some(i) => {
                        i.done = true;
                    },
                    None => {
                        println!("Todo at index {} not found.", index)
                    }
                }
            });
        write_todo_list(&current_todos);
        Self::list_todos(current_todos);
    }

    pub fn edit_todo(mut current_todos: Vec<Todo>, todo_index: usize, todo_new_name: String){
        match current_todos.get_mut(todo_index-1) {
            Some(i) => {
                i.name = todo_new_name;
                write_todo_list(&current_todos);
                Self::list_todos(current_todos);
            },
            None => {
                println!("Todo at index {} not found.", todo_index);
            }
        }
    }

    pub fn remove_todo(mut current_todos: Vec<Todo>, todo_index:usize){
        current_todos.remove(todo_index-1);
        write_todo_list(&current_todos);
        Self::list_todos(current_todos);
    }
}

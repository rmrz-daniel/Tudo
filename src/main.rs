use crate::fs::File;
use std::io::Error;
use std::env::args;
use std::fs::{self, OpenOptions};
use ansi_term::Colour::*;

mod todo;

#[derive(Debug)]
struct Arguments {
    dir: String,
    option: String,
    tasks: Option<Vec<String>>
}

fn main() {
    let args = parse_args(args().collect());
    //Idiomatic rust way of handling
    read_file(args.dir.clone() + "todo.json")
        .map( |file| //Map handles the Ok and leaves the error untouched
            match &args.option.to_lowercase()[..]{
                "init" => eprintln!("{} Usage: {}", Red.paint("Tudo was already initalized."), Blue.paint("Tudo add, remove, list")), 
                "add" => { args.tasks.map_or_else(|| help(), |task| todo::add(args.dir.clone() + "todo.json", file, task));}, //map_or_else is a function for handling Option enums, computes a closure function in the case of None and for the case of Some
                "toggle" => { todo::toggle(args.dir.clone() + "todo.json", file)},
                "clear" => { todo::clear(file)},
                "remove" => { 
                    match args.tasks {
                        None => {todo::remove(args.dir.clone() + "todo.json", file)},
                        Some(task) => {todo::remove_w_args(args.dir.clone() + "todo.json",file, task)}
                    }
                },
                "list" | "ls" => todo::list(file),
                _ => {eprintln!("not an option"); help();} 
            }
        )
        .map_err( |e| //Result map for handling the Error while leaving the Ok() untouched
            match &args.option.to_lowercase()[..] {
                "init" => todo::init(args.dir.clone() + "todo.json"),
                _ => eprintln!("Tudo has not been intialized in this project: {}", e),
            }
        ).ok();
}

fn help() {
    println!("Usage: tudo {}", Blue.paint("{Option} {tasks - if required}"));
    println!("{}", Yellow.paint("-Options:"));
    println!("init - {}", Blue.paint("Initializes tudo for correct directory"));
    println!("add {} - {}", "{tasks}", Blue.paint("Adds tasks to list, tasks need to be enclosed in quotations if longer than one word"));
    println!("remove / remove {} - {}", "{tasks}", Blue.paint("Remove tasks from list, can be done without passing specific tasks"));
    println!("Toggle - {}", Blue.paint("Opens a promp to toggle task as completed or not"));
    println!("list or ls - {}", Blue.paint("lists tasks and their status"));
    println!("clear - {}", Blue.paint("Clears all tasks"));
}

fn read_file(arg: String) -> Result<File, Error>{
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(arg);
    file
}

fn parse_args(args: Vec<String>) -> Arguments {
    //Match the length of 
    match args.len(){
        1 => { 
            help();
            std::process::exit(0);
        },
        2 => Arguments {
            dir: args[0].clone().chars().take(args[0].rfind("\\").unwrap_or(0) + 1).collect(),
            option: args[1].clone(),
            tasks: None,
        },
        _ => Arguments {
            dir: args[0].clone().chars().take(args[0].rfind("\\").unwrap_or(0) + 1).collect(),
            option: args[1].clone(),
            tasks: Some(args[2..].to_vec()),
        }
    }
}
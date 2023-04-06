use std::{fs::File};
use serde::{Deserialize, Serialize};
use ansi_term::{Colour::*, ANSIGenericString};
use inquire::{formatter::MultiOptionFormatter, list_option::ListOption, validator::Validation, MultiSelect, InquireError};


#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
struct Task {
	task: String,
	completed: bool,
}

impl std::fmt::Display for Task {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
    	let status: ANSIGenericString<'_, str> = match self.completed {
    		false => Red.paint(String::from("✗")),
    		true => Green.paint(String::from("✓")),
    	};
        write!(fmt, "{} {}", status, Yellow.paint(&self.task))
    }
}


pub fn init(dir: String) {
	File::create(&dir).unwrap();
	println!("Tudo initialized at: {}", Green.paint(dir))
}

pub fn add(path:String, file: File, tasks: Vec<String>) {
	let mut data: Vec<Task> = get_data(&file);
	file.set_len(0).unwrap();

	tasks.iter()
		.for_each(|task| data.push(Task {
			task: task.to_owned(), 
			completed:false})
		);
	std::fs::write(path, serde_json::to_string_pretty(&data).unwrap() ).unwrap();
	println!("{} {}", Green.paint(tasks.len().to_string()), Green.paint("Task/s added"))
}

pub fn clear(file: File) {
	file.set_len(0).unwrap();
	println!("{}", Green.paint("All tasks were removed"))
}

pub fn remove(path:String, file: File) {
	let mut data: Vec<Task> = get_data(&file);

	match &data.len() {
		0 => {
			eprintln!("{}", Red.paint("You have no tasks!"));
			std::process::exit(0);
		}
		_=>()
	};

    let selected_data = selecter(&data, "removed");

    selected_data.map(|selected| {
    	selected.iter().for_each(|target| {
	    	data.remove(
	    		data.iter().position(|task_in_vec| task_in_vec.task == target.task).unwrap()
	    	);
    	})
    }).map_err(|_| std::process::exit(0)).ok();

    std::fs::write(path, serde_json::to_string_pretty(&data).unwrap() ).unwrap();
}

pub fn toggle(path:String, file: File) {
	let mut data: Vec<Task> = get_data(&file);

	match &data.len() {
		0 => {
			eprintln!("{}", Red.paint("You have no tasks!"));
			std::process::exit(0);
		}
		_=>()
	};

    let selected_data = selecter(&data, "Toggled");

    selected_data.map(|selected| {
    	selected.iter().for_each(|target| {
    		let index_of_target = data.iter().position(|task_in_vec| task_in_vec.task == target.task).unwrap();
	    	data[index_of_target].completed = !data[index_of_target].completed;
    	})
    }).map_err(|_| std::process::exit(0)).ok();

    std::fs::write(path, serde_json::to_string_pretty(&data).unwrap() ).unwrap();

}

pub fn remove_w_args(path:String, file: File, tasks: Vec<String>) {
	let mut data: Vec<Task> = get_data(&file);

	file.set_len(0).unwrap();
	tasks.iter().for_each(|target|{ 
		data.iter().position(|task_in_vec| task_in_vec.task == *target).map_or_else(|| eprintln!("{}", Red.paint("task/s you want to remove do not exist")) , |x|  { data.remove(x); } );
		}
	);
	std::fs::write(path, serde_json::to_string_pretty(&data).unwrap() ).unwrap();

	println!("{} {}", Green.paint(tasks.len().to_string()), Red.paint("Task/s removed"))

}

pub fn list(file: File){
	let data: Vec<Task> = get_data(&file);

	match data.len(){
		0 => println!("{}", Red.paint("No tasks")),
		_ => {data.iter().for_each(|task| println!("{}", task))},
	}

	

}

fn get_data(file: &File) -> Vec<Task> {
	serde_json::from_reader(&(*file)).map_or(Vec::new(), |x| x)
}

fn selecter(data: &Vec<Task>, operation: &str) -> Result<Vec<Task>, InquireError> {
	let validator = |a: &[ListOption<&Task>]| {
        if a.len() < 1 {
             Ok(Validation::Invalid("Must select a task".into()))
        } else {
        	Ok(Validation::Valid)
        }
    };

    let formatter: MultiOptionFormatter<Task> = &|a| format!("{} Task/s were {}", a.len(), Green.paint(operation));

    let ans = MultiSelect::new( &(format!("{}:", Purple.paint("Select tasks to remove")))[..], data.to_vec())
    	.with_validator(validator)
    	.with_formatter(formatter)
        .prompt();
    ans
}
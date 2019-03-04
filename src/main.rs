extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, BufRead, BufReader};

mod entry;

fn main() {
    let command = env::args().nth(1).unwrap_or(String::from(""));

    match command.as_ref() {
        "review" => review(),
        "help" => help(),
        "" => help(),
        _ => {
            println!("Unrecognized command: {}", command);
        }
    }
}

fn help() {
    println!("mojo review - Review past entries.");
}

fn review() {
    let entries = get_daily_entries_for_review();

    if entries.len() == 0 {
        println!("Nothing to review");
    } else {
        for entry in entries.iter() {
            review_entry(entry);
        }
    }
}

fn get_daily_entries_for_review() -> Vec<entry::Entry> {
    let dates = vec![
        "2019-03-01",
        "2019-03-02",
        "2019-03-03",
    ];

    let mut entries = Vec::new();

    for date in dates.iter() {
        let mut file = File::open(format!("fixtures/{}.md", date)).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
        if let Ok(entry) = entry::Entry::from(&contents) {
            entries.push(entry);
        }
    }

    entries
}

enum TaskCommand {
    MarkComplete,
    MoveForward,
    MoveToFutureLog,
    Skip,
    Delete,
    Unknown,
}

fn task_command_from(input: &str) -> TaskCommand {
    match input.trim()  {
        "x" => TaskCommand::MarkComplete,
        ">" => TaskCommand::MoveForward,
        "<" => TaskCommand::MoveToFutureLog,
        "n" => TaskCommand::Skip,
        "d" => TaskCommand::Delete,
        _ => TaskCommand::Unknown,
    }
}

// Review Entry
fn review_entry(entry: &entry::Entry) {
    for task in entry.tasks.iter() {
        review_entry_task(task);
        println!("\n");
    }
}

fn review_entry_task(task: &entry::task::Task) {
    // Print Task
    if task.status != entry::task::TaskStatus::Incomplete {
        return;
    }

    let command = request_task_command(&task);

    match command  {
        TaskCommand::MarkComplete => println!("Done"),
        TaskCommand::MoveForward => println!("Moved forward"),
        TaskCommand::MoveToFutureLog => println!("Moved to future log"),
        TaskCommand::Skip  => println!("Skipped"),
        TaskCommand::Delete => println!("Deleted"),
        TaskCommand::Unknown => println!("?"),
    }
}

fn request_task_command(task: &entry::task::Task) -> TaskCommand {
    print_task_options(task);
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Unable to read input");
    task_command_from(&input)
}

fn print_task_options(task: &entry::task::Task) {
    println!("TODO: {}", task.description);
    println!("x - Mark as complete");
    println!("> - Move forward");
    println!("< - Move to future log");
    println!("n - Skip");
    println!("d - Delete");
}

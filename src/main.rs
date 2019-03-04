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
//
// Review Entry
fn review_entry(entry: &entry::Entry) {
    for task in entry.tasks.iter() {
        review_entry_task(task);
    }
}

fn review_entry_task(task: &entry::task::Task) {
    // Print Task
    println!("Unfinished task found...\n");
    println!("\t{}\n", task.description);
    if task.status == entry::task::TaskStatus::Incomplete {
        // Get Action
        println!("What would you like to do?");
        println!("\tx - Mark as complete");
        println!("\t> - Move forward");
        println!("\t< - Move to future log");
        println!("\tn - Next");
        println!("\td - Delete");
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                // Perform Action
                println!("Moved task forward: {}", input);
            }
            Err(error) => println!("error: {}", error),
        }
    } else {
        // Do nothing
    }
}

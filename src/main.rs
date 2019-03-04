extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

mod task;

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
    let dates_to_review = get_daily_entries_for_review();

    if dates_to_review.len() == 0 {
        println!("Nothing to review");
    } else {
        println!("Reviewing notes since {}", dates_to_review[0].date);

        for entry in dates_to_review.iter() {
            println!("# Reviewing {}", entry.date);
            review_entry(entry);
            println!("\n\n\n");
        }
    }
}

fn get_daily_entries_for_review() -> Vec<DailyEntry> {
    vec![
        DailyEntry::new("2019-03-01"),
        DailyEntry::new("2019-03-02"),
        DailyEntry::new("2019-03-03"),
    ]
}

// DailyEntry
struct DailyEntry {
    date: String,
}

impl DailyEntry {
    pub fn new(date: &str) -> DailyEntry {
        DailyEntry {
            date: String::from(date),
        }
    }
}

// Task
#[derive(PartialEq)]
enum TaskStatus {
    Todo,
}

struct Task<'entry> {
    status: TaskStatus,
    description: &'entry String,
}

impl<'entry> Task<'entry> {
    pub fn from_line(line: &'entry String) -> Result<Task<'entry>, ()> {
        let task_regex = Regex::new("^ *[-*+] *\\[[x<> ]?\\]").unwrap();

        if task_regex.is_match(&line) {
            Ok(Task {
                status: TaskStatus::Todo,
                description: line,
            })
        } else {
            Err(())
        }
    }
}

// Review Entry
fn review_entry(entry: &DailyEntry) {
    let file = File::open(format!("fixtures/{}.md", entry.date)).expect("Entry does not exist");

    for line in BufReader::new(file).lines() {
        let line_contents = line.unwrap();

        match Task::from_line(&line_contents) {
            Ok(task) => review_entry_task(&task),
            Err(_) => {}
        }
    }
}

fn review_entry_task(task: &Task) {
    // Print Task
    println!("Unfinished task found...\n");
    println!("\t{}\n", task.description);
    if task.status == TaskStatus::Todo {
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

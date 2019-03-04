pub mod task;

pub struct Entry {
    pub tasks: Vec<task::Task>
}

impl Entry {
    pub fn from(contents: &str) -> Result<Entry, ()> {
        let mut tasks = Vec::new();

        for line in contents.lines() {
            match task::Task::from_line(line) {
                Ok(task) => tasks.push(task),
                _ => {}
            }
        }

        Ok(Entry { 
            tasks 
        })
    }
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn entry_without_tasks() {
        let entry = Entry::from("").unwrap();

        assert!(entry.tasks.is_empty());
    }

    #[test]
    fn entry_with_task() {
        let task = "Play sports";
        let entry = Entry::from(&format!("* [ ] {}", task).to_string()).unwrap();

        assert_eq!(task, entry.tasks[0].description);
    }

    #[test]
    fn has_a_task() {
        let task = "Eat Samosas";
        let entry = Entry::from(&format!("* [ ] {}", task).to_string()).unwrap();

        assert_eq!(task, entry.tasks[0].description);
    }
}



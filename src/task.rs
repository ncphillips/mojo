use regex::Regex;

#[derive(PartialEq)]
#[derive(Debug)]
enum TaskStatus {
    Incomplete,
    Complete,
    Migrated,
    Backlog,
    Deleted,
}

#[derive(Debug)]
struct Task {
    status: TaskStatus,
    description: String
}

const TASK_RE: &str = "^ *[-*+] *\\[([(x<>\\- ])?\\] (.*)";

impl Task {
    pub fn new(description: &str) -> Task {
        Task {
            status: TaskStatus::Incomplete,
            description: String::from(description),
        }
    }

    pub fn mark_complete(&mut self) {
        self.status = TaskStatus::Complete
    }

    pub fn from_line(description: &str) -> Result<Task, ()> {
        let task_re= Regex::new(TASK_RE).unwrap();
        if !task_re.is_match(description) {
            return Err(())
        }

        let caps = task_re.captures(description).unwrap();

        let status = match &caps[1] {
            "x" => TaskStatus::Complete,
            ">" => TaskStatus::Migrated,
            "<" => TaskStatus::Backlog,
            "-" => TaskStatus::Deleted,
            _ => TaskStatus::Incomplete,
        };

        Ok(Task {
            status,
            description: String::from(&caps[2]),
        })
    }
}

impl PartialEq for Task{
    fn eq(&self, other: &Task) -> bool {
        self.description == other.description
    }
}

#[cfg(test)]
mod new {
    use super::*;

    #[test]
    fn creates_incomplete_tasks() {
        let task = Task::new("Eat bread");

        assert_eq!(TaskStatus::Incomplete, task.status)
    }

}

#[cfg(test)]
mod mark_complete {
    use super::*;

    #[test]
    fn sets_status_to_complete() {
        let mut task = Task::new("Eat bread");

        task.mark_complete();

        assert_eq!(TaskStatus::Complete, task.status)
    }
}

#[cfg(test)]
mod from_line {
    use super::*;

    #[test]
    fn returns_err_on_empty_lines() {
        let task = Task::from_line("");

        assert_eq!(Err(()), task)
    }

    #[test]
    fn returns_task_for_dash() {
        match Task::from_line("- [ ] test") {
            Ok(_) => assert!(true),
            Err(_) => assert!(false, "Expected task to be returned"),
        }
    }

    #[test]
    fn returns_task_for_star() {
        match Task::from_line("* [ ] test") {
            Ok(_) => assert!(true),
            Err(_) => assert!(false, "Expected task to be returned"),
        }
    }

    #[test]
    fn returns_task_for_plus() {
        match Task::from_line("+ [ ] test") {
            Ok(_) => assert!(true),
            Err(_) => assert!(false, "Expected task to be returned"),
        }
    }

    #[test]
    fn loads_description() {
        let task = Task::from_line("+ [ ] test").unwrap();

        assert_eq!("test", task.description)
    }

    #[test]
    fn understands_incomplete_tasks() {
        let task = Task::from_line("+ [ ] test").unwrap();

        assert_eq!(TaskStatus::Incomplete, task.status)
    }

    #[test]
    fn understands_complete_tasks() {
        let task = Task::from_line("+ [x] test").unwrap();

        assert_eq!(TaskStatus::Complete, task.status)
    }

    #[test]
    fn understands_migrated_tasks() {
        let task = Task::from_line("+ [>] test").unwrap();

        assert_eq!(TaskStatus::Migrated, task.status)
    }

    #[test]
    fn understands_backlog_tasks() {
        let task = Task::from_line("+ [<] test").unwrap();

        assert_eq!(TaskStatus::Backlog, task.status)
    }

     #[test]
    fn understands_deleted_tasks() {
        let task = Task::from_line("+ [-] test").unwrap();

        assert_eq!(TaskStatus::Deleted, task.status)
    }
}


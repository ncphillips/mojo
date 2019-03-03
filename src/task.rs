#[derive(PartialEq)]
#[derive(Debug)]
enum TaskStatus {
    Incomplete,
}

struct Task {
    status: TaskStatus,
}

impl Task {
    pub fn new(_description: &str) -> Task {
        Task {
            status: TaskStatus::Incomplete,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_incomplete_by_default() {
        let task = Task::new("Eat bread");

        assert_eq!(TaskStatus::Incomplete, task.status)
    }

}

use std::collections::HashMap;

struct Task {
    name: String,
    priority: i32,
}

struct App {
    tasks: HashMap<String, Task>,
}

impl App {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }

    fn add_task(&mut self, name: &str, priority: i32) {
        let task = Task {
            name: name.to_string(),
            priority,
        };
        self.tasks.insert(name.to_string(), task);
    }

    fn remove_task(&mut self, name: &str) {
        self.tasks.remove(name);
    }

    fn display_tasks(&self) {
        let mut task_names: Vec<&String> = self.tasks.keys().collect();
        task_names.sort_by(|a, b| self.tasks[*b].priority.cmp(&self.tasks[*a].priority));
        println!("Tasks (by priority): ");

        for name in task_names {
            let task = &self.tasks[name];
            println!("{}: {}", task.name, task.priority);
        }
    }
}

fn main() {
    println!("loa main");
}

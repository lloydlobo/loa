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
}

fn main() {
    println!("loa main");
}

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
        println!(
            r#"
|-------------------------------
| id |  Tasks (by priority): 
|-------------------------------"#
        );

        for (i, name) in task_names.iter().enumerate() {
            let task = &self.tasks[*name];
            println!("| {i:#2?} | {}: {}", task.priority, task.name,);
        }
        println!(r#"|-------------------------------"#);
    }
}

fn main() {
    try_main().unwrap();
}

fn try_main() -> anyhow::Result<()> {
    let mut app = App::new();

    HashMap::from([
        ("Take out the trash", 1),
        ("Do laundry", 3),
        ("Clean kitchen", 2),
    ])
    .into_iter()
    .for_each(|(name, priority)| {
        app.add_task(name, priority);
    });

    app.display_tasks();

    app.remove_task("Do laundry");

    app.display_tasks();

    Ok(())
}

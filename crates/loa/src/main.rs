use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs, time::Instant};

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
    config_manager().unwrap();
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

//------------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
struct Config {
    general: General,
    #[serde(rename = "Environment")]
    environment: Environment,
}

impl Config {
    /// Update the values in the Config struct.
    fn update_config(&mut self, other: &Config) {
        self.general.path_to_polars_csv = other.general.path_to_polars_csv.to_owned();
        self.environment.name = other.environment.name.to_owned();
    }
}

#[derive(Serialize, Deserialize)]
struct General {
    path_to_polars_csv: String,
}

#[derive(Serialize, Deserialize)]
struct Environment {
    name: String,
}

const CONFIG_PATH: &str = "crates/loa/src/config.toml";
/// With `config_manager` easily access and modify the frequently used features of your application through the config.toml file.
fn config_manager() -> anyhow::Result<()> {
    {
        // With this approach, you can access the individual values by using the dot notation (e.g., config.General.path_to_polars_csv).
        let mut config: Config = toml::from_str(include_str!("config.toml")).unwrap();
        let path_to_polars_csv = &config.general.path_to_polars_csv.clone();
        let env_name = &config.environment.name.clone();
        println!("Path to polars csv: {}", path_to_polars_csv);
        println!("Environment name: {}", env_name);

        config.update_config(&Config {
            general: General {
                path_to_polars_csv: format!("./{now:?}polars.csv", now = Instant::now()),
            },
            environment: Environment {
                name: "development".to_string(),
            },
        });

        let curr_dir = env::current_dir().unwrap().join(CONFIG_PATH);
        dbg!(&curr_dir);

        // Serialize the `Config` struct to a TOML string.
        let config = toml::to_string(&config).unwrap();
        fs::write(curr_dir, config).unwrap();
    }
    let config: toml::Value = toml::from_str(include_str!("config.toml")).unwrap();
    let path_to_polars_csv = config["general"]["path_to_polars_csv"].as_str().unwrap();
    let env_name = config["Environment"]["name"].as_str().unwrap();

    println!("Path to polars csv: {}", path_to_polars_csv);
    println!("Environment name: {}", env_name);

    Ok(())
}

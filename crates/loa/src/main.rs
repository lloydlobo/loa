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

const CONFIG_FILE_NAME: &str = "config.toml";

/// With `config_manager` easily access and modify the frequently used features of your application through the config.toml file.
///
/// With this approach, you can access the individual values by using the dot notation (e.g., config.General.path_to_polars_csv).
///
/// This code takes the following steps:
///
/// * Determines the current directory with env::current_dir()
/// * Joins the current directory with CONFIG_FILE_NAME to get the full path to the config.toml file
/// * Reads the content of the config.toml file with fs::read_to_string(). If the file does not exist, a default config is created.
/// * Parses the content of the config.toml file with toml::from_str() into a Config struct
/// * Prints the values of the path_to_polars_csv and env_name fields from the Config struct
/// * Updates the Config struct with new values
/// * Serializes the updated Config struct to a TOML string with toml::to_string()
/// * Writes the serialized Config string back to the config.toml file with fs::write()
/// * Returns Ok(()) to indicate success.
//
// let config: toml::Value = toml::from_str(include_str!("config.toml")).unwrap();
// let path_to_polars_csv = config["general"]["path_to_polars_csv"].as_str().unwrap();
// let env_name = config["Environment"]["name"].as_str().unwrap();
// println!("Path to polars csv: {}", path_to_polars_csv);
// println!("Environment name: {}", env_name);
fn config_manager() -> anyhow::Result<()> {
    {
        let curr_dir = env::current_dir().unwrap();
        let path = curr_dir.join(CONFIG_FILE_NAME);
        let config = match fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!(
                    "Creating default config. {} not found: {}",
                    path.to_string_lossy(),
                    anyhow::anyhow!(e)
                );
                r#"[general]
path_to_polars_csv = "./polars.csv"

[Environment]
name = "development""#
                    .to_string()
            }
        };

        let mut config: Config = toml::from_str(&config).unwrap();
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

        // Serialize the `Config` struct to a TOML string.
        let config = toml::to_string(&config).unwrap();
        fs::write(path, config).unwrap();
    }

    Ok(())
}

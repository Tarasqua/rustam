use std::collections::HashMap;

use chrono::prelude::*;

trait Plugin {
    fn execute(&self) -> String;
}

struct GreenPlugin;

impl GreenPlugin {
    fn new() -> Self {
        GreenPlugin
    }
}

impl Plugin for GreenPlugin {
    fn execute(&self) -> String {
        "Executing Green Plugin".to_string()
    }
}

struct TimePlugin;

impl TimePlugin {
    fn new() -> Self {
        TimePlugin
    }
}

impl Plugin for TimePlugin {
    fn execute(&self) -> String {
        let now_utc: DateTime<Utc> = Utc::now();
        let now_local: DateTime<Local> = Local::now();
        format!(
            "UTC Time: {}, Local Time: {}",
            now_utc,
            now_local.format("%Y-%m-%d %H:%M:%S")
        )
    }
}

pub fn make_plugins() {
    let plugins: Vec<Box<dyn Plugin>> = vec![Box::new(GreenPlugin), Box::new(TimePlugin)];

    for plugin in plugins {
        println!("{}", plugin.execute());
    }

    let mut dynamic_plugins: HashMap<String, Box<dyn Plugin>> = HashMap::new();
    dynamic_plugins.insert("Green".to_string(), Box::new(GreenPlugin));
    dynamic_plugins.insert("Time".to_string(), Box::new(TimePlugin));
    for (key, plugin) in &dynamic_plugins {
        println!("{}: {}", key, plugin.execute());
    }
}

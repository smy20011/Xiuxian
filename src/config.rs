use std::fs::{self, File};

use anyhow::Result;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::level::Level;

static CONFIG_DIR: &'static str = "config.json";


// Sequence in form of A_n = A_(n-1) + a * b ^ (n-1)
#[derive(Debug, Serialize, Deserialize)]
pub struct Sequence {
    pub start: u64,
    pub a: u64,
    pub b: u64,
}

impl Sequence {
    pub fn diff(&self, n: usize) -> u64 {
        self.a * self.b.pow((n - 1) as u32)
    }
}

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct Config {
    pub cult_default: u64,
    pub cult_per_year: u64,
    pub lifespan: Sequence,
    pub lvup: Sequence,
    pub spawn_per_year: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            cult_default: 10,
            cult_per_year: 1,
            lifespan: Sequence { start: 100, a: 800, b: 10 },
            lvup: Sequence { start: 10, a: 90, b: 10 },
            spawn_per_year: 100,
        }
    }
}

fn read_config(path: &str) -> Result<Config> {
    let file = File::open(path)?;
    Ok(serde_json::from_reader(file)?)
}

fn write_config(path: &str, config: &Config) -> Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, config)?;
    Ok(())
}

fn read_config_system(mut config: ResMut<Config>) {
    if let Ok(false) = fs::exists(CONFIG_DIR) {
        let _ = write_config(CONFIG_DIR, &Config::default());
    }
    match read_config(CONFIG_DIR) {
        Ok(c) => *config = c,
        Err(e) => info!("Failed to read config from path {}, error: {}", CONFIG_DIR, e.to_string())
    }
    Level::update(&config);
}

pub fn config_plugin(app: &mut App) {
    app.init_resource::<Config>();
    app.add_systems(Startup, read_config_system);
}

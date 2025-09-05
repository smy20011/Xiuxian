mod battle;
mod benchmark;
mod config;
mod cultivation;
mod level;
mod life;
mod spawn;
mod stat;
mod system;

use crate::cultivation::Cultivation;
use crate::level::Level;
use crate::life::Life;

use battle::battle_plugin;
use benchmark::benchmark_system;
use bevy::{log::LogPlugin, prelude::*};
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;
use config::config_plugin;
use cultivation::cultivation_plugin;
use life::life_plugin;
use spawn::spawn_plugin;
use stat::stat_plugin;
use system::game_system;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin::default())
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(config_plugin)
        .add_plugins(game_system)
        .add_plugins(life_plugin)
        .add_plugins(cultivation_plugin)
        .add_plugins(battle_plugin)
        .add_plugins(spawn_plugin)
        .add_plugins(benchmark_system)
        .add_plugins(stat_plugin)
        .run();
}

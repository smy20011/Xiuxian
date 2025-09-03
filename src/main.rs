mod battle;
mod benchmark;
mod cultivation;
mod level;
mod life;
mod spawn;
mod system;

use crate::cultivation::Cultivation;
use crate::level::Level;
use crate::life::Life;

use battle::{Battle, battle_plugin};
use benchmark::benchmark_system;
use bevy::{ecs::query::QueryData, prelude::*, time::common_conditions::on_timer};
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;
use cultivation::cultivation_plugin;
use itertools::Itertools;
use life::life_plugin;
use spawn::spawn_plugin;
use std::{collections::HashMap, time::Duration};
use system::{GamePlay, game_system};

#[derive(Resource, Default)]
struct GlobalState {
    year: u64,
}

fn increase_year(mut state: ResMut<GlobalState>) {
    state.year += 1;
}

#[derive(QueryData)]
struct CultivatorQuery {
    life: &'static Life,
    cultivation: &'static Cultivation,
    battle: &'static Battle,
}

#[derive(Default, Debug)]
struct PerGroupStatistics {
    size: usize,
    courage: f64,
    cultivation: f64,
}

impl PerGroupStatistics {
    fn new<'a, T: IntoIterator<Item = &'a CultivatorQueryItem<'a>>>(
        cultivators: T,
    ) -> PerGroupStatistics {
        let mut result = PerGroupStatistics::default();
        for cult in cultivators {
            result.size += 1;
            result.courage += cult.battle.courage;
            result.cultivation += cult.cultivation.cultivation as f64;
        }
        result.courage /= result.size as f64;
        result.cultivation /= result.size as f64;
        result
    }
}

#[derive(Resource, Default, Debug)]
struct XiuxianStatistics {
    per_level_stat: HashMap<Level, PerGroupStatistics>,
    global_stat: PerGroupStatistics,
}

fn update_stats(query: Query<CultivatorQuery>, mut stats: ResMut<XiuxianStatistics>) {
    let cultivators: Vec<CultivatorQueryItem> = query.iter().collect();
    stats.global_stat = PerGroupStatistics::new(&cultivators);
    stats.per_level_stat = cultivators
        .iter()
        .into_group_map_by(|i| i.cultivation.level)
        .into_iter()
        .map(|(l, c)| (l, PerGroupStatistics::new(c)))
        .collect();
}

fn print_stats(stats: Res<XiuxianStatistics>, state: Res<GlobalState>) {
    println!(
        "现在是第{}年，现有修士{}名，平均勇气值{:.3}，平均修为{:.3}",
        state.year,
        stats.global_stat.size,
        stats.global_stat.courage,
        stats.global_stat.cultivation
    );

    for (level, stat) in stats.per_level_stat.iter().sorted_by_key(|i| i.0) {
        println!(
            "修为{}, 现有修士{}名，平均勇气值{:.3}，平均修为{:.3}",
            level.name(),
            stat.size,
            stat.courage,
            stat.cultivation
        );
    }
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(game_system)
        .add_plugins(life_plugin)
        .add_plugins(cultivation_plugin)
        .add_plugins(battle_plugin)
        .add_plugins(spawn_plugin)
        .add_plugins(benchmark_system)
        .init_resource::<XiuxianStatistics>()
        .init_resource::<GlobalState>()
        .add_systems(
            Update,
            (
                (increase_year).in_set(GamePlay::PreBattle),
                (update_stats, print_stats)
                    .chain()
                    .run_if(on_timer(Duration::from_secs(3)))
                    .in_set(GamePlay::AfterBattle),
            ),
        )
        .run();
}

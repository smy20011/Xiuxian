use bevy::prelude::*;
use bevy::{ecs::query::QueryData, time::common_conditions::on_timer};
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use itertools::Itertools;
use rand::seq::IteratorRandom;
use std::{collections::HashMap, time::Duration};

use crate::cultivation::Cultivation;
use crate::level::Level;
use crate::life::Life;
use crate::spawn::DeathEvent;
use crate::system::GamePlay;
use crate::battle::Battle;

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
    death_age: Vec<u64>,
    total_death: u64,
    death_by_battle: u64,
    death_by_age: u64,
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
    info!(
        "现在是第{}年，现有修士{}名，平均勇气值{:.3}，平均修为{:.3}",
        state.year,
        stats.global_stat.size,
        stats.global_stat.courage,
        stats.global_stat.cultivation
    );

    for (level, stat) in stats.per_level_stat.iter().sorted_by_key(|i| i.0) {
        info!(
            "修为{}, 现有修士{}名，平均勇气值{:.3}，平均修为{:.3}",
            level.name(),
            stat.size,
            stat.courage,
            stat.cultivation
        );
    }
    let total_death = stats.death_age.len();
    let sum: u64 = stats.death_age.iter().sum();
    info!("死亡人数: {}，平均寿命: {}", stats.total_death, sum as f64/ total_death as f64);
    info!("战斗死亡: {}，年老死亡: {}", stats.death_by_battle, stats.death_by_age);
}

fn collect_death(mut ev_death: EventReader<DeathEvent>, mut stats: ResMut<XiuxianStatistics>, mut rng: GlobalEntropy<WyRand>) {
    for ev in ev_death.read() {
        stats.death_age.push(ev.life.age);
        stats.total_death += 1;
        if ev.life.lifespan == ev.life.age {
            stats.death_by_age += 1;
        } else {
            stats.death_by_battle += 1;
        }
    }
    if stats.death_age.len() > 2000 {
        stats.death_age = stats.death_age.iter().cloned().choose_multiple(&mut rng, 1000);
    }
}

pub fn stat_plugin(app: &mut App) {
    app.init_resource::<XiuxianStatistics>()
        .init_resource::<GlobalState>()
        .add_systems(
            Update,
            (
                (increase_year, collect_death).in_set(GamePlay::PreBattle),
                (update_stats, print_stats)
                    .chain()
                    .run_if(on_timer(Duration::from_secs(3)))
                    .in_set(GamePlay::AfterBattle),
            ),
        );
}

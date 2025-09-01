mod emulation;

use crate::emulation::player::Level;
use bevy::{ecs::query::QueryData, prelude::*};
use bevy_prng::WyRand;
use bevy_rand::{global::GlobalEntropy, plugin::EntropyPlugin};
use itertools::Itertools;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Component, Debug)]
struct Life {
    age: u64,
    lifespan: u64,
    alive: bool,
}

fn increase_age(query: Query<&mut Life>) {
    for mut life in query {
        life.age += 1
    }
}

fn mark_death(query: Query<&mut Life>) {
    for mut life in query {
        if life.lifespan <= life.age {
            life.alive = false
        }
    }
}

#[derive(Component, Debug)]
struct Cultivation {
    level: Level,
    cultivation: u64,
}

impl Cultivation {
    fn try_advance(query: Query<(&mut Cultivation, &mut Life)>) {
        for (mut cult, mut life) in query {
            if let Some(next_level) = cult.level.next_level()
                && cult.cultivation >= next_level.required_cultivation()
            {
                cult.level = next_level;
                life.lifespan = next_level.total_lifespan();
            }
        }
    }

    fn increase_cultivation(query: Query<&mut Cultivation>) {
        for mut cult in query {
            cult.cultivation += 1;
        }
    }

    fn get_win_rate(&self, opponent: &Self) -> f64 {
        self.cultivation as f64 / (self.cultivation + opponent.cultivation) as f64
    }
}

#[derive(Component)]
struct Battle {
    courage: f64,
}

#[derive(QueryData)]
#[query_data(mutable)]
struct BattleQuery {
    cultivation: &'static mut Cultivation,
    battle: &'static Battle,
    life: &'static mut Life,
    entity: Entity,
}

fn will_battle(a: &BattleQueryItem, b: &BattleQueryItem) -> bool {
    let win_rate = a.cultivation.get_win_rate(&b.cultivation);
    a.battle.courage > 1.0 - win_rate
}

fn battle(mut rng: GlobalEntropy<WyRand>, mut query: Query<BattleQuery>) {
    let mut players: Vec<BattleQueryItem> = query.iter_mut().collect();
    let mut battles: Vec<[usize; 2]> = Vec::new();
    players.shuffle(&mut rng);

    for i in 0..(players.len() / 2) {
        let a = &players[i];
        let b = &players[players.len() - i - 1];
        if a.cultivation.level == b.cultivation.level && (will_battle(a, b) || will_battle(b, a)) {
            battles.push([i, players.len() - i - 1]);
        }
    }

    for p in battles {
        let prob: f64 = rng.random();
        let mut pair = p.clone();
        if prob
            > players[p[0]]
                .cultivation
                .get_win_rate(&players[p[1]].cultivation)
        {
            pair.reverse();
        }
        let [winner, loser] = players.get_disjoint_mut(pair).unwrap();
        winner.cultivation.cultivation += loser.cultivation.cultivation / 10;
        loser.life.alive = false;
        if loser.cultivation.cultivation > 10000 {
            println!("设置死亡{:?}, {:?}", loser.life, loser.cultivation);
            println!("对手{:?}, {:?}", winner.life, winner.cultivation);
        }
    }
}

#[derive(Bundle)]
struct Cultivator {
    life: Life,
    cultivation: Cultivation,
    battle: Battle,
}

struct World {}

impl World {
    fn spawn_cultivators(mut command: Commands, mut rng: GlobalEntropy<WyRand>) {
        for _ in 0..100 {
            command.spawn(Cultivator {
                life: Life {
                    age: 20,
                    lifespan: 100,
                    alive: true,
                },
                cultivation: Cultivation {
                    level: Level::Foundation,
                    cultivation: 10,
                },
                battle: Battle {
                    courage: rng.random(),
                },
            });
        }
    }

    fn despawn_dead(mut commands: Commands, query: Query<(Entity, CultivatorQuery)>) {
        for (entity, c) in query {
            if !c.life.alive {
                commands.entity(entity).despawn();
            }
        }
    }
}

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
    if state.year % 10 == 0 {
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
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .init_resource::<XiuxianStatistics>()
        .init_resource::<GlobalState>()
        .add_systems(
            Update,
            (
                World::spawn_cultivators,
                (
                    increase_age,
                    increase_year,
                    Cultivation::increase_cultivation,
                ),
                battle,
                Cultivation::try_advance,
                mark_death,
                World::despawn_dead,
                update_stats,
                print_stats,
            )
                .chain(),
        )
        .run();
}

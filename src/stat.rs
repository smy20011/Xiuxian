use bevy::prelude::*;
use bevy::{ecs::query::QueryData, time::common_conditions::on_timer};
use core::f64;
use itertools::Itertools;
use std::{collections::HashMap, time::Duration};

use crate::battle::Courage;
use crate::cultivation::Cultivation;
use crate::level::Level;
use crate::life::Life;
use crate::spawn::DeathEvent;
use crate::system::GamePlay;

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
    courage: &'static Courage,
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
            result.courage += cult.courage.courage;
            result.cultivation += cult.cultivation.cultivation as f64;
        }
        result.courage /= result.size as f64;
        result.cultivation /= result.size as f64;
        result
    }
}

#[derive(Debug, Default)]
struct Average {
    total: usize,
    average: f64,
}

impl Average {
    fn digiest(&mut self, data: f64) {
        self.average = (self.average * self.total as f64 + data) / (self.total + 1) as f64;
        self.total += 1;
    }
}

#[derive(Resource, Default, Debug)]
struct XiuxianStatistics {
    per_level_stat: HashMap<Level, PerGroupStatistics>,
    global_stat: PerGroupStatistics,
    death: Average,
    death_by_battle: Average,
    death_by_age: Average,
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
    info!(
        "死亡人数: {}，平均寿命: {}",
        stats.death.total, stats.death.average
    );
    info!(
        "战斗死亡: {}，平均寿命: {}",
        stats.death_by_battle.total, stats.death_by_battle.average
    );
    info!(
        "年老死亡: {}，平均寿命: {}",
        stats.death_by_age.total, stats.death_by_age.average
    );
}

fn collect_death(mut ev_death: EventReader<DeathEvent>, mut stats: ResMut<XiuxianStatistics>) {
    for ev in ev_death.read() {
        let age = ev.life.age as f64;
        stats.death.digiest(age);
        if ev.life.lifespan == ev.life.age {
            stats.death_by_age.digiest(age);
        } else {
            stats.death_by_battle.digiest(age);
        }
    }
}

pub fn stat_plugin(app: &mut App) {
    app.init_resource::<XiuxianStatistics>()
        .init_resource::<GlobalState>()
        .add_systems(
            Update,
            (
                (increase_year, collect_death).in_set(GamePlay::Spawn),
                (update_stats, print_stats)
                    .chain()
                    .run_if(on_timer(Duration::from_secs(3)))
                    .in_set(GamePlay::Finish),
            ),
        );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_per_group_statistics() {
        let mut world = World::new();
        world.spawn((
            Life { age: 20, lifespan: 100, alive: true },
            Cultivation { level: Level::Foundation, cultivation: 10 },
            Courage { courage: 0.5 },
        ));
        world.spawn((
            Life { age: 20, lifespan: 100, alive: true },
            Cultivation { level: Level::Foundation, cultivation: 20 },
            Courage { courage: 0.7 },
        ));

        let mut query = world.query::<CultivatorQuery>();
        let items: Vec<_> = query.iter(&world).collect();
        let stats = PerGroupStatistics::new(&items);

        assert_eq!(stats.size, 2);
        assert_eq!(stats.courage, 0.6);
        assert_eq!(stats.cultivation, 15.0);
    }

    #[test]
    fn test_average() {
        let mut avg = Average::default();
        avg.digiest(10.0);
        avg.digiest(20.0);
        avg.digiest(30.0);
        assert_eq!(avg.total, 3);
        assert_eq!(avg.average, 20.0);
    }

    #[test]
    fn test_collect_death() {
        let mut app = App::new();
        app.add_event::<DeathEvent>();
        app.init_resource::<XiuxianStatistics>();
        app.add_systems(Update, collect_death);

        let mut death_events = app.world_mut().resource_mut::<Events<DeathEvent>>();
        death_events.send(DeathEvent {
            life: Life { age: 100, lifespan: 100, alive: false },
        });
        death_events.send(DeathEvent {
            life: Life { age: 50, lifespan: 120, alive: false },
        });

        app.update();

        let stats = app.world().resource::<XiuxianStatistics>();
        assert_eq!(stats.death.total, 2);
        assert_eq!(stats.death.average, 75.0);
        assert_eq!(stats.death_by_age.total, 1);
        assert_eq!(stats.death_by_age.average, 100.0);
        assert_eq!(stats.death_by_battle.total, 1);
        assert_eq!(stats.death_by_battle.average, 50.0);
    }
}

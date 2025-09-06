use bevy::{ecs::query::QueryData, prelude::*};
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use itertools::Itertools;
use rand::Rng;
use rand::seq::SliceRandom;

use crate::cultivation::Cultivation;
use crate::life::Life;
use crate::system::GamePlay;

#[derive(Component)]
pub struct Courage {
    pub courage: f64,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct BattleQuery {
    cultivation: &'static mut Cultivation,
    battle: &'static Courage,
    life: &'static mut Life,
    entity: Entity,
}

fn will_battle(a: &BattleQueryReadOnlyItem, b: &BattleQueryReadOnlyItem) -> bool {
    let win_rate = a.cultivation.get_win_rate(&b.cultivation);
    a.battle.courage > 1.0 - win_rate
}

#[derive(Resource, Default)]
struct BattlePair(Vec<(Entity, Entity)>);

fn pair(query: Query<BattleQuery>, mut rng: GlobalEntropy<WyRand>, mut pairs: ResMut<BattlePair>) {
    let mut players: Vec<_> = query.iter().map(|i| i.entity).collect();
    players.shuffle(&mut rng);
    pairs.0 = players.chunks_exact(2).map(|l| (l[0], l[1])).collect();
}

fn filter_battle(data: Query<BattleQueryReadOnly>, mut pairs: ResMut<BattlePair>) {
    pairs.0 = pairs
        .0
        .iter()
        .filter(|(a, b)| {
            let (a, b) = (data.get(*a).unwrap(), data.get(*b).unwrap());
            if a.cultivation.level != b.cultivation.level {
                return false;
            }
            will_battle(&a, &b) || will_battle(&b, &a)
        })
        .cloned()
        .collect();
}

fn battle(mut rng: GlobalEntropy<WyRand>, mut data: Query<BattleQuery>, battles: Res<BattlePair>) {
    for pair in &battles.0 {
        let prob: f64 = rng.random();
        let [mut winner, mut loser] = data.get_many_mut([pair.0, pair.1]).unwrap();
        if prob > winner.cultivation.get_win_rate(&loser.cultivation) {
            (winner, loser) = (loser, winner);
        }
        winner.cultivation.cultivation += loser.cultivation.cultivation / 10;
        loser.life.alive = false;
    }
}

pub fn battle_plugin(app: &mut App) {
    app.init_resource::<BattlePair>();
    app.add_systems(
        Update,
        (
            battle.in_set(GamePlay::Battle),
            pair.in_set(GamePlay::Pair),
            filter_battle.in_set(GamePlay::FilterPair),
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::level::Level;

    #[test]
    fn test_will_battle() {
        let mut world = World::new();
        let a_entity = world.spawn((
            Cultivation {
                level: Level::Foundation,
                cultivation: 100,
            },
            Courage { courage: 0.8 },
            Life { age: 0, lifespan: 100, alive: true },
        )).id();
        let b_entity = world.spawn((
            Cultivation {
                level: Level::Foundation,
                cultivation: 100,
            },
            Courage { courage: 0.2 },
            Life { age: 0, lifespan: 100, alive: true },
        )).id();

        let mut query = world.query::<BattleQuery>();
        let a = query.get(&world, a_entity).unwrap();
        let b = query.get(&world, b_entity).unwrap();

        assert!(will_battle(&a, &b));
        assert!(!will_battle(&b, &a));
    }
}

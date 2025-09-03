use bevy::{ecs::query::QueryData, prelude::*};
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::system::GamePlay;
use crate::cultivation::Cultivation;
use crate::life::Life;

#[derive(Component)]
pub struct Battle {
    pub courage: f64,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct BattleQuery {
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
    }
}


pub fn battle_plugin(app: &mut App) {
    app.add_systems(Update, battle.in_set(GamePlay::Battle));
}

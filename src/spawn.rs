use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use rand::Rng;

use crate::Battle;
use crate::Cultivation;
use crate::Level;
use crate::Life;
use crate::system::GamePlay;

#[derive(Bundle)]
struct Cultivator {
    life: Life,
    cultivation: Cultivation,
    battle: Battle,
}

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

fn despawn_dead(mut commands: Commands, query: Query<(Entity, &Life)>) {
    for (entity, life) in query {
        if !life.alive {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            spawn_cultivators.in_set(GamePlay::PreBattle),
            despawn_dead.in_set(GamePlay::AfterBattle),
        ),
    );
}

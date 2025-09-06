use bevy::prelude::*;
use bevy::state::commands;
use bevy_prng::WyRand;
use bevy_rand::global::GlobalEntropy;
use rand::Rng;

use crate::Cultivation;
use crate::Level;
use crate::Life;
use crate::battle::Courage;
use crate::config::Config;
use crate::system::GamePlay;

#[derive(Bundle)]
struct Cultivator {
    life: Life,
    cultivation: Cultivation,
    courage: Courage,
}

#[derive(Event)]
pub struct DeathEvent {
    pub life: Life,
}

fn spawn_cultivators(mut command: Commands, mut rng: GlobalEntropy<WyRand>, config: Res<Config>) {
    let mut cultiavors: Vec<Cultivator> = Vec::with_capacity(config.spawn_per_year);
    for _ in 0..config.spawn_per_year {
        cultiavors.push(Cultivator {
            life: Life {
                age: 20,
                lifespan: 100,
                alive: true,
            },
            cultivation: Cultivation {
                level: Level::Foundation,
                cultivation: 10,
            },
            courage: Courage {
                courage: rng.random(),
            },
        });
    }
    command.spawn_batch(cultiavors);
}

fn despawn_dead(
    mut commands: Commands,
    query: Query<(Entity, &Life)>,
    mut ev_death: EventWriter<DeathEvent>,
) {
    for (entity, life) in query {
        if !life.alive {
            commands.entity(entity).despawn();
            ev_death.write(DeathEvent { life: life.clone() });
        }
    }
}

pub fn spawn_plugin(app: &mut App) {
    app.add_event::<DeathEvent>();
    app.add_systems(
        Update,
        (
            spawn_cultivators.in_set(GamePlay::Spawn),
            despawn_dead.in_set(GamePlay::Finish),
        ),
    );
}

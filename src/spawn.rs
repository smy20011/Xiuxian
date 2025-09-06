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

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_rand::plugin::EntropyPlugin;

    #[test]
    fn test_spawn_cultivators() {
        let mut app = App::new();
        app.add_plugins(EntropyPlugin::<WyRand>::default());
        app.init_resource::<Config>();
        app.add_systems(Update, spawn_cultivators);
        app.update();

        let mut query = app.world_mut().query::<(&Life, &Cultivation, &Courage)>();
        let config = app.world().get_resource::<Config>().unwrap();
        assert_eq!(query.iter(app.world()).count(), config.spawn_per_year);
    }

    #[test]
    fn test_despawn_dead() {
        let mut app = App::new();
        app.add_event::<DeathEvent>();
        let dead_entity = app.world_mut().spawn(Life { age: 100, lifespan: 100, alive: false }).id();
        let alive_entity = app.world_mut().spawn(Life { age: 50, lifespan: 100, alive: true }).id();

        app.add_systems(Update, despawn_dead);
        app.update();

        assert!(app.world().get_entity(dead_entity).is_err());
        assert!(app.world().get_entity(alive_entity).is_ok());
    }
}


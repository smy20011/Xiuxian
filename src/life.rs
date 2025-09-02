use bevy::prelude::*;

use crate::system::GamePlay;

#[derive(Component, Debug)]
pub struct Life {
    pub age: u64,
    pub lifespan: u64,
    pub alive: bool,
}

impl Life {
    pub fn increase_age(query: Query<&mut Life>) {
        for mut life in query {
            life.age += 1;
            if life.lifespan <= life.age {
                life.alive = false;
            }
        }
    }
}

pub fn life_plugin(app: &mut App) {
    app.add_systems(Update, Life::increase_age.in_set(GamePlay::AfterBattle));
}

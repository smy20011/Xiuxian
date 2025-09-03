use crate::level::Level;
use crate::life::Life;
use crate::system::GamePlay;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Cultivation {
    pub level: Level,
    pub cultivation: u64,
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

    pub fn get_win_rate(&self, opponent: &Self) -> f64 {
        self.cultivation as f64 / (self.cultivation + opponent.cultivation) as f64
    }
}

pub fn cultivation_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            Cultivation::try_advance.in_set(GamePlay::PreBattle),
            Cultivation::increase_cultivation.in_set(GamePlay::AfterBattle),
        ),
    );
}

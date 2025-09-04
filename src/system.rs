use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GamePlay {
    Spawn,
    Pair,
    Battle,
    Finish,
}

pub fn game_system(app: &mut App) {
    app.configure_sets(
        Update,
        (
            GamePlay::Pair.after(GamePlay::Spawn),
            GamePlay::Battle.after(GamePlay::Pair),
            GamePlay::Finish.after(GamePlay::Battle),
        ),
    );
}

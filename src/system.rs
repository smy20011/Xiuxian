use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GamePlay {
    Spawn,
    Pair,
    FilterPair,
    Battle,
    Finish,
}

pub fn game_system(app: &mut App) {
    app.configure_sets(
        Update,
        (
            GamePlay::Pair.after(GamePlay::Spawn),
            GamePlay::FilterPair.after(GamePlay::Pair),
            GamePlay::Battle.after(GamePlay::FilterPair),
            GamePlay::Finish.after(GamePlay::Battle),
        ),
    );
}

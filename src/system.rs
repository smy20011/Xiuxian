use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GamePlay {
    PreBattle,
    Battle,
    AfterBattle,
}

pub fn game_system(app: &mut App) {
    app.configure_sets(
        Update,
        (
            GamePlay::Battle.after(GamePlay::PreBattle),
            GamePlay::AfterBattle.after(GamePlay::Battle),
        ),
    );
}

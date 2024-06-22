use bevy::prelude::*;

#[derive(Debug, Clone, Default, Hash, Eq, PartialEq, States)]
pub enum MainState {
    #[default]
    LoadAssets,
    Game,
}

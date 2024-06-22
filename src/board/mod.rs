use bevy::{prelude::*, utils::HashMap};

use crate::{states::MainState, vectors::Vector2Int};

pub mod components;
mod systems;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .add_system(systems::spawn_map.in_schedule(OnEnter(MainState::Game)));
    }
}

#[derive(Default, Resource)]
pub struct Board {
    pub tiles: HashMap<Vector2Int, Entity>,
}

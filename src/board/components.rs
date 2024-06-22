use bevy::prelude::*;

use crate::vectors::Vector2Int;

// Keep position component separate to reuse for other game objects. ex. items
#[derive(Component)]
pub struct Position {
    pub v: Vector2Int,
}

#[derive(Component)]
pub struct Tile;

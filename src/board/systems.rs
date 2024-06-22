use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::vectors::Vector2Int;

use super::components::{Position, Tile};
use super::Board;

pub fn spawn_map(mut commands: Commands, mut board: ResMut<Board>) {
    board.tiles = HashMap::new();
    const BOARD_WIDTH: i32 = 8;
    const BOARD_HEIGHT: i32 = 8;
    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            let v = Vector2Int::new(x, y);
            let tile = commands.spawn((Position { v }, Tile)).id();
            board.tiles.insert(v, tile);
        }
    }
}

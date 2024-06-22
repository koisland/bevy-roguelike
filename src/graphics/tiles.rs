use bevy::prelude::*;

use super::{GraphicsAssets, TILE_SIZE};
use crate::board::components::{Position, Tile};

// Couple sprites with tile entity.
pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Tile>>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, position) in query.iter() {
        // TODO: Test code. Remove later.
        let mut sprite = TextureAtlasSprite::new(177);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        sprite.color = Color::OLIVE;
        // v3 with position of entity scaled by tile size.
        let v = Vec3::new(
            TILE_SIZE * position.v.x as f32,
            TILE_SIZE * position.v.y as f32,
            0.0,
        );
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture_atlas: assets.sprite_texture.clone(),
            transform: Transform::from_translation(v),
            ..Default::default()
        });
    }
}

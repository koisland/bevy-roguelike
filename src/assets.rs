use bevy::asset::LoadState;
use bevy::prelude::*;

use crate::states::MainState;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetList>()
            .add_system(check_asset_loading.in_set(OnUpdate(MainState::LoadAssets)));
    }
}

#[derive(Default, Resource)]
pub struct AssetList(pub Vec<HandleUntyped>);

pub fn check_asset_loading(
    asset_server: Res<AssetServer>,
    asset_list: Res<AssetList>,
    mut next_state: ResMut<NextState<MainState>>,
) {
    let asset_group_state = asset_server.get_group_load_state(asset_list.0.iter().map(|a| a.id()));
    match asset_group_state {
        LoadState::Loaded => next_state.set(MainState::Game),
        LoadState::Failed => {
            error!("Failed to load assets.")
        }
        _ => {}
    }
}

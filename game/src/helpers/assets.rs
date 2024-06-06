use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct AssetsLoading(Vec<UntypedHandle>);

impl AssetsLoading {
    pub fn add(&mut self, handle: UntypedHandle) {
        self.0.push(handle);
    }

    pub fn add_all(&mut self, handles: Vec<UntypedHandle>) {
        self.0.extend(handles);
    }
}

pub fn wait_for_assets(server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) -> bool {
    use bevy::asset::LoadState;

    let finished = loading
        .0
        .iter()
        .all(|h| server.get_load_state(h) == Some(LoadState::Loaded));

    if finished {
        loading.0.clear();
    }

    return finished;
}

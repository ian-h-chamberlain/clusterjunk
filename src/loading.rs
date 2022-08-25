use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_rapier2d::prelude::*;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>()
                .continue_to_state(GameState::Menu),
        )
        .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(load_doodads));
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
}

pub struct DoodadAssets {
    pub square: DoodadAsset,
}

pub struct DoodadAsset {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
    pub collider: Collider,
}

fn load_doodads(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let square = {
        let mesh = meshes.add(shape::Cube::new(100.0).into());
        let material = materials.add(ColorMaterial {
            color: Color::BLUE,
            ..default()
        });
        let collider = Collider::cuboid(50.0, 50.0);

        DoodadAsset {
            mesh,
            material,
            collider,
        }
    };

    commands.insert_resource(DoodadAssets { square });
}

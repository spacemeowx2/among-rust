use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, log::prelude::*, prelude::*};
use components::{Moving, Position};
use entities::create_player;
use plugins::input::InputPlugin;
use systems::player_move::player_move;

mod components;
mod entities;
mod plugins;
mod systems;

#[bevy_main]
fn main() {
    let mut app = App::build();
    app.add_resource(Msaa { samples: 4 });
    app.add_plugins(DefaultPlugins);
    app.add_plugin(FrameTimeDiagnosticsPlugin::default());
    app.add_plugin(InputPlugin::default());
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.add_startup_system(setup.system());
    app.add_system(animate_sprite_system.system());
    app.add_system(player_move.system());
    app.add_system(print_player_system.system());
    app.run();
}

fn print_player_system(player: Query<(Entity, &Position)>) {
    for (_, pos) in player.iter() {
        info!("position {} {}", pos.0.x, pos.0.y)
    }
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &Moving,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle, moving) in query.iter_mut() {
        if moving.0 {
            timer.tick(time.delta_seconds());
            if timer.finished() {
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            }
        } else {
            sprite.index = 0;
        }
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // commands.spawn(Camera2dBundle::default());

    create_player(commands)
        .with_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::new(3.0, 3.0, 1.0)),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 100.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(100.0, 100.0, 100.0)),
            ..Default::default()
        });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 100.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_translation(Vec3::new(100.0, 100.0, 100.0)),
        ..Default::default()
    });

    commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 5.0, 0.1)),
        ..Default::default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0))
            .looking_at(Vec3::default(), Vec3::unit_y()),
        ..Default::default()
    });
}

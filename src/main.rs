use bevy::prelude::*;
use bevy_mod_picking::*;

mod pieces;
mod board;
mod ui;

use pieces::*;
use crate::board::BoardPlugin;
use crate::ui::UIPlugin;

fn main() {
    App::build()
        .add_resource(Msaa{samples: 4}) // anti-aliasing enabled
        .add_resource(WindowDescriptor{               // setup window
            title: "LVI Chess".to_string(),
            width: 1600.,
            height: 1600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(PiecesPlugin)
        .add_plugin(UIPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 17.0, 4.0),
            )),
            ..Default::default()
        })
        .with(PickSource::default())
        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}



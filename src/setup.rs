use crate::components;
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // terrain
    commands.spawn(PbrBundle {
        mesh: meshes.add(Rectangle::new(10.0, 5.0)),
        material: materials.add(Color::linear_rgb(0.0, 1.0, 0.1)),
        // material: materials.add(StandardMaterial {
        //     base_color_texture: Some(asset_server.load("grid_shader.wgsl")), // Texture placeholder
        //     ..Default::default()
        // }),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // players
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder::new(0.2, 1.0)),
            material: materials.add(Color::linear_rgb(1.0, 1.0, 1.0)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        components::Player {
            name: String::from("Noé"),
        },
        components::Direction::new(),
        components::Velocity::new(),
        components::BallOwner,
    ));

    // ball
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.1)),
            material: materials.add(Color::linear_rgb(1.0, 0.0, 0.0)),
            transform: Transform::from_xyz(1.0, 0.2, 0.0),
            ..default()
        },
        components::Ball,
    ));

    // lighting
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 1.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, 0.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

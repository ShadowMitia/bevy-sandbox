use bevy::prelude::*;

#[derive(Component)]
struct Image;

fn setup_system(mut commands: Commands, windows: Res<Windows>, asset_server: Res<AssetServer>) {
    let window = windows.get_primary().unwrap();
    let size = 64.0;
    let window_size = Vec2::new(window.width(), window.height());

    let width = window_size.x / size;
    let height = f32::ceil(window_size.y / size);

    commands.spawn(Camera2dBundle::default());

    let tex = asset_server.load("line.png");

    let mut all_sprites = Vec::new();

    for j in 0..(height as u32) {
        for i in 0..(width as u32) {
            all_sprites.push((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        size * i as f32 - window_size.x / 2.0 + size / 2.0,
                        size * j as f32 - window_size.y / 2.0,
                        0.0,
                    ),
                    texture: tex.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(size, size)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Image,
            ));
        }
    }

    commands.spawn_batch(all_sprites);
}

#[derive(Resource)]
struct RepeatTimer(Timer);

fn zero_or_one() -> f32 {
    if rand::random() {
        1.0
    } else {
        0.0
    }
}

fn update_sprites(
    time: Res<Time>,
    mut timer: ResMut<RepeatTimer>,
    mut query: Query<(&Image, &mut Transform)>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        for (_, mut transform) in query.iter_mut() {
            transform.rotate(Quat::from_rotation_z(
                std::f32::consts::PI / 2.0 * zero_or_one(),
            ));
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "10Print".into(),
                ..Default::default()
            },
            ..Default::default()
        }))
        .insert_resource(RepeatTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_startup_system(setup_system)
        .add_system(update_sprites)
        .run();
}

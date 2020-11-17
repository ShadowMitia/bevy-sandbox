use bevy::prelude::*;

struct Image;

fn setup_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title("10 Print".to_string());

    let window_width = window.width();
    let window_height = window.height();

    let size = 128.0;

    let width = window_width as f32 / size;
    let height = window_height as f32 / size;

    let half_width = (window_width as f32 + 0.5)/ 2.0 ;
    let half_height = (window_height as f32 + 0.5) / 2.0;

    let texture_handle = asset_server.load("line.png");

    let mat = materials.add(texture_handle.into());

    commands
        .spawn(UiCameraComponents::default())
        .spawn(Camera2dComponents::default());

    for j in 0..(height as u32) {
        for i in 0..(width as u32) {
            let zero_or_one = if rand::random() { 1.0 } else { 0.0 };

            commands
                .spawn(SpriteComponents {
                    material: mat.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        half_width - size * i as f32 - size / 2.0,
                        half_height - size * j as f32 - size / 2.0,
                        0.0,
                    )) * Transform::from_rotation(Quat::from_rotation_z(
                        (std::f32::consts::PI / 2.0) * zero_or_one,
                    )),
                    sprite: Sprite::new(Vec2::new(size, size)),
                    ..Default::default()
                })
                .with(Image);
        }
    }
}

struct UpdateTimer(Timer);

fn update_sprites(
    time: Res<Time>,
    mut timer: ResMut<UpdateTimer>,
    mut query: Query<(&Image, &mut Transform)>,
) {
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        for (_, mut transform) in query.iter_mut() {
            let zero_or_one = if rand::random() { 1.0 } else { 0.0 };

            transform.rotate(Quat::from_rotation_z(
                std::f32::consts::PI / 2.0 * zero_or_one,
            ));
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(UpdateTimer(Timer::from_seconds(1.0, true)))
        .add_startup_system(setup_system.system())
        .add_system(update_sprites.system())
        .run();
}

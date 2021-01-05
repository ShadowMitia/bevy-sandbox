use bevy::{prelude::*, sprite::collide_aabb::Collision};


fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let blue = Color::BLUE;

    let _pink = Color::PINK;

    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default())
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(32.0, 32.0)),
            ..Default::default()
        })
        .with(Pacman {
            velocity: Vec2::new(0.0, 0.0),
        })
        .spawn(SpriteBundle {
            material: materials.add(blue.into()),
            transform: Transform::from_translation(Vec3::new(100.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(32.0, 32.0)),
            ..Default::default()
        })
        .with(Wall)
        .spawn(SpriteBundle {
            material: materials.add(blue.into()),
            transform: Transform::from_translation(Vec3::new(-100.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(32.0, 32.0)),
            ..Default::default()
        })
        .with(Wall)
        .with_children(|commands| {
            commands.spawn(SpriteBundle {
                material: materials.add(_pink.into()),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                sprite: Sprite::new(Vec2::new(16.0, 16.0)),
                ..Default::default()
            });
        })
        .spawn(SpriteBundle {
            material: materials.add(blue.into()),
            transform: Transform::from_translation(Vec3::new(-300.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(3.0, 200.0)),
            ..Default::default()
        })
        .with(Wall)
        .spawn(SpriteBundle {
            material: materials.add(blue.into()),
            transform: Transform::from_translation(Vec3::new(-400.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(200.0, 200.0)),
            ..Default::default()
        })
        .with(Wall)
        .spawn(SpriteBundle {
            material: materials.add(_pink.into()),
            transform: Transform::from_translation(Vec3::new(-168.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(200.0, 200.0)),
            ..Default::default()
        })
        .with(Wall);
}

struct Pacman {
    velocity: Vec2,
}

struct Wall;

fn pacman_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Pacman, &mut Transform)>,
) {
    let mut velocity = Vec2::new(0.0, 0.0);
    if keyboard_input.pressed(KeyCode::Left) {
        velocity.x = -1.0;
    } else if keyboard_input.pressed(KeyCode::Right) {
        velocity.x = 1.0;
    } else if keyboard_input.pressed(KeyCode::Up) {
        velocity.y = 1.0;
    } else if keyboard_input.pressed(KeyCode::Down) {
        velocity.y = -1.0;
    }

    for (mut pacman, mut pacman_transform) in query.iter_mut() {
        if velocity.length() > 0.0 {
            pacman.velocity = velocity;
        }

        // move the paddle horizontally
        pacman_transform.translation.x += time.delta_seconds() * pacman.velocity.x * 200.0;
        pacman_transform.translation.y += time.delta_seconds() * pacman.velocity.y * 200.0;
    }
}

// Assumes top left sprite coordinate system
fn aabb_aabb_collide(one: (&Transform, &Sprite), two: (&Transform, &Sprite)) -> bool {
    let one_position = one.0.translation;
    let two_position = two.0.translation;

    let one_size = one.1.size;
    let two_size = two.1.size;

    let one_half_size = one.1.size / 2.0;
    let two_half_size = two.1.size / 2.0;

    let one_position = Vec3::new(
        one_position.x - one_half_size.x,
        one_position.y - one_half_size.y,
        0.0,
    );
    let two_position = Vec3::new(
        two_position.x - two_half_size.x,
        two_position.y - two_half_size.y,
        0.0,
    );

    // collision x-axis?
    let collision_x = one_position.x + one_size.x >= two_position.x
        && two_position.x + two_size.x >= one_position.x;
    // collision y-axis?
    let collision_y = one_position.y + one_size.y >= two_position.y
        && two_position.y + two_size.y >= one_position.y;

    collision_x && collision_y
}

// Assumes center sprite coordinate system
fn aabb_aabb_resolution(
    one: (&Transform, &Sprite),
    two: (&Transform, &Sprite),
) -> (Collision, f32) {
    let one_position = one.0.translation;
    let two_position = two.0.translation;

    let one_size = one.1.size;
    let two_size = two.1.size;

    let one_half_size = Vec3::new(one_size.x / 2.0, one_size.y / 2.0, 0.0);
    let two_half_size = Vec3::new(two_size.x / 2.0, two_size.y / 2.0, 0.0);

    let max_a: Vec3 = one_position + Vec3::new(one_half_size.x, one_half_size.y, 0.0);
    let min_a: Vec3 = one_position - Vec3::new(one_half_size.x, one_half_size.y, 0.0);
    let max_b: Vec3 = two_position + Vec3::new(two_half_size.x, two_half_size.y, 0.0);
    let min_b: Vec3 = two_position - Vec3::new(two_half_size.x, two_half_size.y, 0.0);

    let distances = vec![
        max_b.x - min_a.x,
        max_a.x - min_b.x,
        max_b.y - min_a.y,
        max_a.y - min_b.y,
    ];

    let mut penetration = std::f32::INFINITY;
    let mut best_axis = 10;

    for (i, &d) in distances.iter().enumerate() {
        if d < penetration {
            penetration = d;
            best_axis = i;
        }
    }

    let col = match best_axis {
        0 => Collision::Left,
        1 => Collision::Right,
        2 => Collision::Bottom,
        3 => Collision::Top,
        _ => panic!("Shouldn't happen"),
    };

    (col, penetration)
}

fn pacman_collision(
    mut query: Query<(&mut Pacman, &mut Transform, &Sprite)>,
    query_walls: Query<(&Wall, &Transform, &Sprite)>,
) {
    for (_, wall_transform, wall_sprite) in query_walls.iter() {
        for (mut pacman, mut pacman_transform, pacman_sprite) in query.iter_mut() {
            let collided = aabb_aabb_collide(
                (&pacman_transform, pacman_sprite),
                (wall_transform, wall_sprite),
            );

            if collided {
                // aabb_aabb_collide((wall_transform, wall_sprite), (&pacman_transform, pacman_sprite))
                pacman.velocity = Vec2::new(0.0, 0.0);

                let (collision, diff) = aabb_aabb_resolution(
                    (&mut pacman_transform, pacman_sprite),
                    (wall_transform, wall_sprite),
                );

                let diff = f32::abs(diff) + 0.1;

                match collision {
                    Collision::Left => {
                        pacman_transform.translation.x += diff;
                    }
                    Collision::Right => {
                        pacman_transform.translation.x -= diff;
                    }
                    Collision::Top => {
                        pacman_transform.translation.y -= diff;
                    }
                    Collision::Bottom => {
                        pacman_transform.translation.y += diff;
                    }
                }
            }
        }
    }
}

fn main() {

    App::build()
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(pacman_movement.system())
        .add_system(pacman_collision.system())
        .run();
}

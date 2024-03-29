use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use spatial_partition::{QuadTree, AABB};

mod boids;

#[derive(Component)]
struct Square;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Debug, Component)]
struct Boid {
    id: usize,
}

mod spatial_partition;

fn boids_system(
    mut commands: Commands,
    mut boids: Local<boids::Boids>,
    // mut boids_partition: ResMut<QuadTree>,
    mut boid_sprites: Query<(&Boid, &mut Transform)>,
    windows: Res<Windows>,
) {
    const NUM_BOIDS: usize = 100;

    let mut boids_partition = QuadTree::new(AABB::new(Vec2::ZERO, Vec2::new(800.0, 800.0)));

    if boids.size() < 10 {
        let width = 4.0;
        let height = 4.0;

        for i in 0..NUM_BOIDS {
            let d = rand::random::<f32>();
            let dd = rand::random::<f32>();
            let r = d * std::f32::consts::TAU;
            // let x = (rand::random::<f32>() * 2.0 - 1.0) * r * 300.0;
            // let y = (rand::random::<f32>() * 2.0 - 1.0) * r * 300.0;

            let x = f32::sin(r) * 2.0 - 1.0;
            let y = f32::cos(r) * 2.0 - 1.0;

            let x = x * 200.0 * dd;
            let y = y * 200.0 * dd;

            boids.add(Vec2::new(x, y));

            boids_partition.insert(Vec2::new(x, y), i);

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(width as f32, height as f32)),
                        color: Color::RED,
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                    ..Default::default()
                },
                Boid { id: i },
            ));
        }
    } else {
        for i in 0..boids.positions.len() {
            boids_partition.insert(boids.positions[i], i);
        }
    }

    // // println!("{:#?}", *boids_partition);

    // let sprites = boids
    //     .positions
    //     .iter()
    //     .enumerate()
    //     .map(|(id, pos)| {
    //         (
    //             SpriteBundle {
    //                 sprite: Sprite::new(Vec2::new(width as f32, height as f32)),
    //                 material: mat.clone(),
    //                 transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
    //                 ..Default::default()
    //             },
    //             Boid { id },
    //         )
    //     })
    //     .collect::<Vec<(SpriteBundle, Boid)>>();

    // // println!("{:#?}", sprites.len());

    // commands.spawn_batch(sprites);

    let window = windows.get_primary().unwrap();

    let half_window_width = window.width() / 2.0;
    let half_window_height = window.height() / 2.0;

    let mut new_accelerations = Vec::new();
    new_accelerations.resize_with(boids.accelerations.len(), || Vec2::ZERO);

    for id in 0..boids.size() {
        let boid = boids.positions[id];

        let data = boids_partition
            .query_range(&AABB::new(Vec2::new(boid.x, boid.y), Vec2::new(30.0, 30.0)));

        if data.is_empty() {
            continue;
        }

        let mut accelerations = data
            .iter()
            .map(|(_, bid)| boids.accelerations[*bid])
            .collect::<Vec<Vec2>>();

        let velocities = data
            .iter()
            .map(|(_, bid)| boids.velocities[*bid])
            .collect::<Vec<Vec2>>();

        let positions = data.iter().map(|(pos, _)| *pos).collect::<Vec<Vec2>>();

        // println!("{:#?} {:#?}", data, boid);

        let index = data.iter().position(|&(_, bid)| bid == id).unwrap();

        boids::Boids::flock(
            accelerations.as_mut_slice(),
            velocities.as_slice(),
            positions.as_slice(),
        );

        // new_accelerations.extend(accelerations.iter().map(|&acc| acc * time.delta_seconds()));
        new_accelerations[id] = accelerations[index];
    }

    boids.accelerations = new_accelerations;

    boids.update();

    // println!("{:#?}", boids_partition);

    // *boids_partition = QuadTree::new(boids_partition.boundary);

    for (boid, mut transform) in boid_sprites.iter_mut() {
        // *boids_partition = QuadTree::new(boids_partition.boundary);

        // boids_partition.remove(boids.positions[boid.id]);

        let x = boids.positions[boid.id].x;
        let y = boids.positions[boid.id].y;

        if x < -half_window_width {
            boids.positions[boid.id].x = half_window_width;
        } else if x > half_window_width {
            boids.positions[boid.id].x = -half_window_width;
        }

        if y < -half_window_height {
            boids.positions[boid.id].y = half_window_height;
        } else if y > half_window_height {
            boids.positions[boid.id].y = -half_window_height;
        }

        transform.translation.x = boids.positions[boid.id].x;
        transform.translation.y = boids.positions[boid.id].y;

        // boids_partition.insert(boids.positions[boid.id], boid.id);

        // let root = &*boids_partition;

        // let nw = &root.children.as_ref().unwrap()[0];
        // let ne = &root.children.as_ref().unwrap()[1];
        // let sw = &root.children.as_ref().unwrap()[2];
        // let se = &root.children.as_ref().unwrap()[3];

        // let bounds = &root.boundary;

        // let square = shapes::RegularPolygon {
        //     sides: 4,
        //     center: Vec2::new(0.0, 0.0),
        //     feature: shapes::RegularPolygonFeature::SideLength(200.0),
        // };

        // commands
        // .spawn(square.draw(
        //     materials.add(ColorMaterial::color(Color::rgba_linear(0.0, 1.0, 0.0, 0.5))),
        //     TessellationMode::Fill(FillOptions::default()),
        //     Transform {
        //         translation: Vec3::new(0.0, 0.0, 0.0),
        //         ..Default::default()
        //     },
        // ))
        // .with(Square);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugin(ShapePlugin)
        // // Adds frame time diagnostics
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // // Adds a system that prints diagnostics to the console
        //        .add_plugin(PrintDiagnosticsPlugin::default())
        // // Any plugin can register diagnostics
        // // Uncomment this to add some render resource diagnostics:
        // .add_plugin(bevy::wgpu::diagnostic::WgpuResourceDiagnosticsPlugin::default())
        // // Uncomment this to add an entity count diagnostics:
        // .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
        // // Uncomment this to add an asset count diagnostics:
        // .add_plugin(bevy::asset::diagnostic::AssetCountDiagnosticsPlugin::<
        //     ColorMaterial,
        // >::default())
        // .add_resource(QuadTree::new(AABB::new(Vec2::zero(), Vec2::new(800.0, 800.0))) as QuadTree)
        .add_startup_system(setup)
        .add_system(boids_system)
        .run();
}

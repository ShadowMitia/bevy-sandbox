use bevy::{prelude::*, DefaultPlugins};
use spatial_partition::QuadTree;

mod boids {

    use super::*;
    #[derive(Debug, Default)]
    pub struct Boids {
        pub positions: Vec<Vec2>,
        pub velocities: Vec<Vec2>,
        pub accelerations: Vec<Vec2>,
    }

    impl Boids {
        pub fn size(&self) -> usize {
            self.positions.len()
        }

        pub fn add(&mut self, position: Vec2) {
            self.positions.push(position);

            let x = rand::random::<f32>() * 2.0 - 1.0;
            let y = rand::random::<f32>() * 2.0 - 1.0;
            let vel = Vec2::new(x, y);
            self.velocities.push(vel);
            self.accelerations.push(Vec2::zero());
        }

        pub fn update(&mut self) {
            let triple_iter = self
                .velocities
                .iter_mut()
                .zip(self.accelerations.iter_mut())
                .zip(self.positions.iter_mut())
                .map(|((a, b), c)| (a, b, c));

            for (velocity, acceleration, position) in triple_iter {
                *velocity += *acceleration;

                if velocity.length() > Boids::MAX_SPEED {
                    *velocity = velocity.normalize() * Boids::MAX_SPEED;
                }
                *position += *velocity;
                *acceleration = Vec2::zero();
            }
        }

        const MAX_SPEED: f32 = 10.0;
        const MAX_FORCE: f32 = 0.5;

        fn separate(velocities: &[Vec2], positions: &[Vec2]) -> Vec<Vec2> {
            let desired_seperation = 25.0;

            let mut steers = Vec::new();

            for i in 0..positions.len() {
                let mut steer = Vec2::zero();
                let mut count = 0;

                for j in 0..positions.len() {
                    let dist = (positions[i] - positions[j]).length();
                    if dist > 0.0 && dist < desired_seperation {
                        let diff = positions[i] - positions[j];
                        let diff = diff.normalize();
                        let diff = diff / dist;
                        steer += diff;
                        count += 1;
                    }
                }

                if count > 0 {
                    steer /= count as f32;
                }

                if steer.length() > 0.0 {
                    steer = steer.normalize();
                    steer *= Boids::MAX_SPEED;
                    steer -= velocities[i];
                    if steer.length() > Boids::MAX_FORCE {
                        steer = steer.normalize();
                        steer *= Boids::MAX_FORCE;
                    }
                }

                steers.push(steer);
            }

            steers
        }

        fn align(velocities: &[Vec2], positions: &[Vec2]) -> Vec<Vec2> {
            let neighbor_dist = 50.0;

            let mut aligns = Vec::new();

            for i in 0..positions.len() {
                let mut sum = Vec2::zero();
                let mut count = 0;

                for j in 0..positions.len() {
                    let dist = (positions[i] - positions[j]).length();
                    if dist > 0.0 && dist < neighbor_dist {
                        sum += velocities[j];
                        count += 1;
                    }
                }

                if count > 0 {
                    sum /= count as f32;

                    sum = sum.normalize();
                    sum *= Boids::MAX_SPEED;
                    let mut steer = sum - velocities[i];
                    if steer.length() > Boids::MAX_FORCE {
                        steer = steer.normalize();
                        steer *= Boids::MAX_FORCE;
                    }
                    aligns.push(steer);
                } else {
                    aligns.push(Vec2::zero());
                }
            }

            aligns
        }

        fn cohesion(velocities: &[Vec2], positions: &[Vec2]) -> Vec<Vec2> {
            let neighbor_dist = 50.0;

            let mut cohesions = Vec::new();

            for i in 0..positions.len() {
                let mut sum = Vec2::zero();
                let mut count = 0;

                for j in 0..positions.len() {
                    let dist = (positions[i] - positions[j]).length();
                    let dist = dist.abs();
                    if dist > 0.0 && dist < neighbor_dist {
                        sum += positions[j];
                        count += 1;
                    }
                }

                if count > 0 {
                    sum /= count as f32;
                    cohesions.push(Boids::seek(sum, positions[i], velocities[i]));
                } else {
                    cohesions.push(Vec2::zero());
                }
            }

            cohesions
        }

        fn seek(target: Vec2, position: Vec2, velocity: Vec2) -> Vec2 {
            let desired = target - position;
            let desired = desired.normalize() * Boids::MAX_SPEED;

            let mut steer = desired - velocity;

            if steer.length() > Boids::MAX_FORCE {
                steer = steer.normalize();
                steer *= Boids::MAX_FORCE;
            }
            steer
        }

        pub fn flock(
            accelerations: &mut [bevy::prelude::Vec2],
            velocities: &[Vec2],
            positions: &[Vec2],
        ) {
            let mut sep = Boids::separate(velocities, positions);
            let mut ali = Boids::align(velocities, positions);
            let mut coh = Boids::cohesion(velocities, positions);

            for i in 0..sep.len() {
                sep[i] *= 1.5;
                ali[i] *= 1.0;
                coh[i] *= 1.0;

                accelerations[i] += sep[i];
                accelerations[i] += ali[i];
                accelerations[i] += coh[i];
            }
        }
    }
}

fn setup(commands: &mut Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}

#[derive(Debug)]
struct Boid {
    id: usize,
}

#[derive(Debug, Default)]
pub struct AABB {
    center: Vec2,
    half_dimension: f32,
}

impl AABB {
    pub fn new(center: Vec2, half_dimension: f32) -> Self {
        Self {
            center,
            half_dimension,
        }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x <= self.center.x + self.half_dimension
            && point.x >= self.center.x - self.half_dimension
            && point.y <= self.center.y + self.half_dimension
            && point.y >= self.center.y - self.half_dimension
    }

    pub fn intersects(&self, range: &AABB) -> bool {
        let one_position = range.center;
        let two_position = self.center;

        let one_size = range.half_dimension * 2.0;
        let two_size = self.half_dimension * 2.0;

        let one_half_size = range.half_dimension;
        let two_half_size = self.half_dimension;

        let one_position = Vec3::new(
            one_position.x - one_half_size,
            one_position.y - one_half_size,
            0.0,
        );
        let two_position = Vec3::new(
            two_position.x - two_half_size,
            two_position.y - two_half_size,
            0.0,
        );

        // collision x-axis?
        let collision_x = one_position.x + one_size >= two_position.x
            && two_position.x + two_size >= one_position.x;
        // collision y-axis?
        let collision_y = one_position.y + one_size >= two_position.y
            && two_position.y + two_size >= one_position.y;

        collision_x && collision_y
    }
}

mod spatial_partition {
    use super::*;

    #[derive(Debug, Default)]
    pub struct QuadTree {
        points: Vec<(Vec2, usize)>,
        boundary: AABB,
        children: [Option<Box<QuadTree>>; 4],
    }

    impl QuadTree {
        pub fn new(boundary: AABB) -> Self {
            Self {
                points: Vec::new(),
                boundary,
                children: [None, None, None, None],
            }
        }

        pub fn insert(&mut self, position: Vec2, data: usize) -> bool {
            if !self.boundary.contains(position) {
                return false;
            }

            if self.children[0].is_none() && self.points.len() < 4 {
                self.points.push((position, data));
                return true;
            }

            if self.children[0].is_none() {
                self.subdivide();
            }

            if self.children[0].as_mut().unwrap().insert(position, data) {
                return true;
            }

            if self.children[1].as_mut().unwrap().insert(position, data) {
                return true;
            }

            if self.children[2].as_mut().unwrap().insert(position, data) {
                return true;
            }

            if self.children[3].as_mut().unwrap().insert(position, data) {
                return true;
            }

            false
        }

        pub fn remove(&mut self, position: Vec2) {
            if !self.boundary.contains(position) {
                return;
            }
            if let Some(id) = self.points.iter().position(|&(pos, _)| pos == position) {
                self.points.remove(id);
            } else {
                if self.children[0].is_none() {
                    return;
                }
                self.children[0].as_mut().unwrap().remove(position);
                self.children[1].as_mut().unwrap().remove(position);
                self.children[2].as_mut().unwrap().remove(position);
                self.children[3].as_mut().unwrap().remove(position);

                if self.children[0].as_ref().unwrap().points.is_empty()
                    && self.children[1].as_ref().unwrap().points.is_empty()
                    && self.children[2].as_ref().unwrap().points.is_empty()
                    && self.children[3].as_ref().unwrap().points.is_empty()
                {
                    self.children[0] = None;
                    self.children[1] = None;
                    self.children[2] = None;
                    self.children[3] = None;
                }
            }
        }

        fn subdivide(&mut self) {
            let half_half = self.boundary.half_dimension / 2.0;
            let nw = AABB::new(
                self.boundary.center - Vec2::new(-half_half, half_half),
                half_half,
            );
            let ne = AABB::new(
                self.boundary.center - Vec2::new(half_half, half_half),
                half_half,
            );
            let sw = AABB::new(
                self.boundary.center - Vec2::new(-half_half, -half_half),
                half_half,
            );
            let se = AABB::new(
                self.boundary.center - Vec2::new(half_half, -half_half),
                half_half,
            );

            self.children = [
                Some(Box::new(Self::new(nw))),
                Some(Box::new(Self::new(ne))),
                Some(Box::new(Self::new(sw))),
                Some(Box::new(Self::new(se))),
            ]
        }

        pub fn query_range(&self, range: &AABB) -> Vec<usize> {
            if self.boundary.intersects(&range) {
                let mut points_in_range = Vec::new();

                for point in &self.points {
                    if range.contains(point.0) {
                        points_in_range.push(point.1);
                    }
                }

                if self.children[0].is_some() {
                    for i in 0..4 {
                        points_in_range
                            .extend(self.children[i].as_ref().unwrap().query_range(&range));
                    }
                }

                points_in_range
            } else {
                Vec::new()
            }
        }
    }

    #[cfg(test)]
    mod quadtree_tests {

        use super::*;

        #[test]
        fn test() {
            let boundary = AABB::new(Vec2::zero(), 200.0);
            let mut quadtree = QuadTree::new(boundary);

            quadtree.insert(Vec2::new(10.0, 10.0), 42);

            assert_eq!(
                quadtree
                    .query_range(&AABB::new(Vec2::new(0.0, 0.0), 20.0))
                    .is_empty(),
                false
            );
            assert_eq!(
                quadtree
                    .query_range(&AABB::new(Vec2::new(0.0, 0.0), 5.0))
                    .is_empty(),
                true
            );
        }

        #[test]
        fn test_remove() {
            let boundary = AABB::new(Vec2::zero(), 200.0);
            let mut quadtree = QuadTree::new(boundary);

            quadtree.insert(Vec2::new(42.0, 10.0), 42);
            quadtree.insert(Vec2::new(15.0, 10.0), 42);
            quadtree.insert(Vec2::new(30.0, 10.0), 42);
            quadtree.insert(Vec2::new(20.0, 10.0), 42);
            quadtree.insert(Vec2::new(10.0, 10.0), 42);

            println!("{:#?}", quadtree);

            assert_eq!(
                quadtree
                    .query_range(&AABB::new(Vec2::new(0.0, 0.0), 20.0))
                    .is_empty(),
                false
            );

            quadtree.remove(Vec2::new(10.0, 10.0));
            println!("Removed!");
            println!("{:#?}", quadtree);

            assert_eq!(
                quadtree
                    .query_range(&AABB::new(Vec2::new(10.0, 10.0), 1.0))
                    .is_empty(),
                true
            );
        }
    }
}

fn boids_system(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut boids: Local<boids::Boids>,
    mut boids_partition: ResMut<QuadTree>,
    mut boid_sprites: Query<(&Boid, &mut Transform)>,
    windows: Res<Windows>,
) {
    const NUM_BOIDS: usize = 10_000;

    if boids.size() < 10 {
        let width = 4.0;
        let height = 4.0;

        for i in 0..NUM_BOIDS {
            let r = rand::random::<f32>();
            let x = (rand::random::<f32>() * 2.0 - 1.0) * r * 300.0;
            let y = (rand::random::<f32>() * 2.0 - 1.0) * r * 300.0;

            boids.add(Vec2::new(x, y));

            boids_partition.insert(Vec2::new(x, y), i);

            commands
                .spawn(SpriteBundle {
                    sprite: Sprite::new(Vec2::new(width as f32, height as f32)),
                    material: materials.add(ColorMaterial::color(Color::RED)),
                    transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                    ..Default::default()
                })
                .with(Boid { id: i });
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
    }

    let window = windows.get_primary().unwrap();

    let half_window_width = window.width() / 2.0;
    let half_window_height = window.height() / 2.0;

    for id in 0..boids.size() {
        let boid = boids.positions[id];

        let data = boids_partition.query_range(&AABB::new(Vec2::new(boid.x, boid.y), 30.0));

        let mut accelerations = data
            .iter()
            .map(|&data| boids.accelerations[data as usize])
            .collect::<Vec<Vec2>>();
        let velocities = data
            .iter()
            .map(|&data| boids.velocities[data as usize])
            .collect::<Vec<Vec2>>();
        let positions = data
            .iter()
            .map(|&data| boids.positions[data as usize])
            .collect::<Vec<Vec2>>();

        boids::Boids::flock(
            accelerations.as_mut_slice(),
            velocities.as_slice(),
            positions.as_slice(),
        );

        data.iter()
            .zip(accelerations)
            .for_each(|(&id, acc)| boids.accelerations[id as usize] = acc);

        boids_partition.remove(boids.positions[id]);
    }

    boids.update();

    for (boid, mut transform) in boid_sprites.iter_mut() {
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
        boids_partition.insert(boids.positions[boid.id], boid.id);
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(QuadTree::new(AABB::new(Vec2::zero(), 800.0)) as QuadTree)
        .add_startup_system(setup.system())
        .add_system(boids_system.system())
        .run();
}

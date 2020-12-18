use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    prelude::*,
    DefaultPlugins,
};

mod boids {

    use super::*;
    #[derive(Default)]
    pub struct Boids {
        pub positions: Vec<Vec2>,
        velocities: Vec<Vec2>,
        accelerations: Vec<Vec2>,
    }

    impl Boids {
        // pub fn new() -> Self {
        //     Self {
        //         positions: Vec::new(),
        //         velocities: Vec::new(),
        //         accelerations: Vec::new(),
        //         ranges: Vec::new(),
        //     }
        // }

        pub fn size(&self) -> usize {
            self.positions.len()
        }

        pub fn add(&mut self, position: Vec2) {
            self.positions.push(position);
            let vel = Vec2::new(rand::random::<f32>(), rand::random::<f32>()) * 10.0;
            self.velocities.push(vel);
            self.accelerations.push(Vec2::zero());
        }

        pub fn update(&mut self) {
            for i in 0..self.positions.len() {
                let velocity = &mut self.velocities[i];
                let acceleration = &mut self.accelerations[i];
                let position = &mut self.positions[i];

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

        fn separate(&mut self) -> Vec<Vec2> {
            let desired_seperation = 25.0;

            let mut steers = Vec::new();

            for i in 0..self.positions.len() {
                let mut steer = Vec2::zero();
                let mut count = 0;

                for j in 0..self.positions.len() {
                    let dist = (self.positions[i] - self.positions[j]).length();
                    let dist = dist.abs();
                    if dist > 0.0 && dist < desired_seperation {
                        let diff = self.positions[i] - self.positions[j];
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
                    steer -= self.velocities[i];
                    if steer.length() > Boids::MAX_FORCE {
                        steer = steer.normalize();
                        steer *= Boids::MAX_FORCE;
                    }
                }

                steers.push(steer);
            }

            steers
        }

        fn align(&mut self) -> Vec<Vec2> {
            let neighbor_dist = 50.0;

            let mut aligns = Vec::new();

            for i in 0..self.positions.len() {
                let mut sum = Vec2::zero();
                let mut count = 0;

                for j in 0..self.positions.len() {
                    let dist = (self.positions[i] - self.positions[j]).length();
                    let dist = dist.abs();
                    if dist > 0.0 && dist < neighbor_dist {
                        sum += self.velocities[j];
                        count += 1;
                    }
                }

                if count > 0 {
                    sum /= count as f32;

                    sum = sum.normalize();
                    sum *= Boids::MAX_SPEED;
                    let mut steer = sum - self.velocities[i];
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

        fn cohesion(&mut self) -> Vec<Vec2> {
            let neighbor_dist = 50.0;

            let mut aligns = Vec::new();

            for i in 0..self.positions.len() {
                let mut sum = Vec2::zero();
                let mut count = 0;

                for j in 0..self.positions.len() {
                    let dist = (self.positions[i] - self.positions[j]).length();
                    let dist = dist.abs();
                    if dist > 0.0 && dist < neighbor_dist {
                        sum += self.positions[j];
                        count += 1;
                    }
                }

                if count > 0 {
                    sum /= count as f32;
                    aligns.push(Boids::seek(sum, self.positions[i], self.velocities[i]));
                } else {
                    aligns.push(Vec2::zero());
                }
            }

            aligns
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
        pub fn flock(&mut self) {
            let mut sep = self.separate();
            let mut ali = self.align();
            let mut coh = self.cohesion();

            for i in 0..sep.len() {
                sep[i] *= 1.5;
                ali[i] *= 1.0;
                coh[i] *= 1.0;

                self.accelerations[i].x += sep[i].x;
                self.accelerations[i].y += sep[i].y;

                self.accelerations[i].x += ali[i].x;
                self.accelerations[i].y += ali[i].y;

                self.accelerations[i].x += coh[i].x;
                self.accelerations[i].y += coh[i].y;
            }
        }
    }
}

fn setup(commands: &mut Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}

struct Boid {
    id: usize,
}

fn boids_system(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut boids: Local<boids::Boids>,
    mut boid_sprites: Query<(&Boid, &mut Transform)>,
    windows: Res<Windows>,
) {
    let boids = &mut *boids;
    let window = windows.get_primary().unwrap();

    let half_window_width = window.width() / 2.0;
    let half_window_height = window.height() / 2.0;

    if boids.size() < 200 {
        boids.add(Vec2::new(0.0, 0.0));

        let width = 4.0;
        let height = 4.0;

        commands
            .spawn(SpriteBundle {
                sprite: Sprite::new(Vec2::new(width as f32, height as f32)),
                material: materials.add(ColorMaterial::color(Color::RED)),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..Default::default()
            })
            .with(Boid {
                id: boids.size() - 1,
            });
    }
    boids.flock();
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
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        // Adds frame time diagnostics
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // Adds a system that prints diagnostics to the console
        // .add_plugin(PrintDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(boids_system.system())
        .run();
}

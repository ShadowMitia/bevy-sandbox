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
        self.accelerations.push(Vec2::ZERO);
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
            *acceleration = Vec2::ZERO;
        }
    }

    const MAX_SPEED: f32 = 8.0;
    const MAX_FORCE: f32 = 1.0;

    fn separate(velocities: &[Vec2], positions: &[Vec2]) -> Vec<Vec2> {
        let desired_seperation = 25.0;

        let mut steers = Vec::new();

        for i in 0..positions.len() {
            let mut steer = Vec2::ZERO;
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
        let neighbor_dist = 100.0;

        let mut aligns = Vec::new();

        for i in 0..positions.len() {
            let mut sum = Vec2::ZERO;
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
                aligns.push(Vec2::ZERO);
            }
        }

        aligns
    }

    fn cohesion(velocities: &[Vec2], positions: &[Vec2]) -> Vec<Vec2> {
        let neighbor_dist = 75.0;

        let mut cohesions = Vec::new();

        for i in 0..positions.len() {
            let mut sum = Vec2::ZERO;
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
                cohesions.push(Vec2::ZERO);
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

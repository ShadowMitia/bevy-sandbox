// From https://observablehq.com/@makio135/creative-coding

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

/*
 * Converts an HSL color value to RGB. Conversion formula
 * adapted from https://en.wikipedia.org/wiki/HSL_and_HSV#HSL_to_RGB
 * Assumes h, s, and l are contained in the set [0, 1] and
 * returns r, g, and b in the set [0, 255].
*/

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let c = 1.0 - f32::abs(2.0 * l - 1.0) * s;

    let h_prime = h / (60.0 / 360.0);

    let x = c * (1.0 - f32::abs(h_prime.rem_euclid(2.0) - 1.0));

    let between = |val, lower, higher| lower <= val && val <= higher;

    let (r, g, b) = if between(h_prime, 0.0, 1.0) {
        (c, x, 0.0)
    } else if between(h_prime, 1.0, 2.0) {
        (x, c, 0.0)
    } else if between(h_prime, 2.0, 3.0) {
        (0.0, c, x)
    } else if between(h_prime, 3.0, 4.0) {
        (0.0, x, c)
    } else if between(h_prime, 4.0, 5.0) {
        (x, 0.0, c)
    } else if between(h_prime, 5.0, 6.0) {
        (c, 0.0, x)
    } else {
        (0.0, 0.0, 0.0)
    };

    let m = l - (c / 2.0);

    (r + m, g + m, b + m)
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    for i in 0..=10 {
        let hsl = hsl_to_rgb(i as f32 * (36.0 / 255.0), 1.0, 0.5);

        let circle = shapes::Circle {
            radius: 100.0,
            center: Vec2::new(25.0 + i as f32 * 60.0 - (10.0 * 60.0) / 2.0, 0.0),
        };

        commands.spawn(GeometryBuilder::build_as(
            &circle,
            DrawMode::Fill(FillMode {
                color: Color::hsl(hsl.0, hsl.1, hsl.2),
                options: FillOptions::default(),
            }),
            Transform::default(),
        ));
    }
}

fn circle_update_system(
    time: Res<Time>,
    mut circles: Query<(&Sprite, &mut Transform)>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    const TAU: f32 = std::f32::consts::TAU;
    const PI: f32 = std::f32::consts::PI;
    let elapsed_time = time.elapsed_seconds() as f32;

    let n = 20.0;

    for (i, (_sprite, mut transform)) in circles.iter_mut().enumerate() {
        let x = (i as f32 / n) * window.width() / 4.0
            + f32::sin(i as f32 / 10.0 + elapsed_time * TAU * 2.0) * 150.0;
        let y = f32::sin((i as f32 / n) * PI * 2.0 + elapsed_time * TAU) * 145.0;

        println!("{} {}", x, y);

        // let r = 55.0 + f32::cos(i as f32 / n * PI * 2.0 + time * TAU) * 45.0;

        transform.translation.x = x;
        transform.translation.y = y;

        // TODO: modify radius with "r * 2.0";
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_system(circle_update_system)
        .run();
}

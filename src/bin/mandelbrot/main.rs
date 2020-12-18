use bevy::{
    prelude::*,
    render::texture::{Extent3d, TextureFormat},
    DefaultPlugins,
};
use image::ImageBuffer;
use num_complex::Complex32;

/*

https://www.youtube.com/watch?v=FFftmWSzgmk

Mandelbrot set are contained in a circle of radius 2, hence the -2,2, and if a value is found outside of -2,2, then it is divergente (should be X^2 + Y^2 < radius).

*/

struct Mandelbrot {}

impl Mandelbrot {
    fn new() -> Self {
        Self {}
    }

    fn get(&self, c: Complex32, iterations: u32) -> (Complex32, u32) {
        let mut zn = Complex32::new(0.0, 0.0);
        let mut iteration = 0;
        while iteration < iterations && zn.norm() <= 2.0 {
            zn = mandelbrot(zn, c);
            iteration += 1;
        }

        (zn, iteration)
    }
}

fn complex_from_coord(x: f32, y: f32) -> Complex32 {
    Complex32::new(x, y)
}

fn mandelbrot(zn: Complex32, c: Complex32) -> Complex32 {
    zn * zn + c
}

fn setup(commands: &mut Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}   

fn mandelbrot_generation_system(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    window: Res<Windows>,
) {
    let mandelbrot = Mandelbrot::new();

    let window = window.get_primary().unwrap();

    let width = window.width() as u32;
    let height = window.height() as u32;

    let scalex = 3.0 / width as f32;
    let scaley = 3.0 / height as f32;

    let max_iterations = 45;

    let mut image = ImageBuffer::new(width, height);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let cx = (x as f32) as f32 * scalex - 2.0;
        let cy = (y as f32) as f32 * scaley - 1.5;

        let (_, i) = mandelbrot.get(complex_from_coord(cx, cy), max_iterations);

        if i < max_iterations {
            let x = x as f32;
            let y = y as f32;

            let radius = 2.0f32;

            let i = i as f32 + 1.0 - ((x * x + y * y).ln() - (2.0f32).ln()).ln() / (radius).ln();

            *pixel = image::Rgba([
                ((i as f32 / max_iterations as f32) * 255.0) as u8,
                ((i as f32 / max_iterations as f32) * 255.0) as u8,
                ((i as f32 / max_iterations as f32) * 255.0) as u8,
                255 as u8,
            ]);
        } else {
            *pixel = image::Rgba([
                ((x as f32 / width as f32) * 255.0) as u8,
                ((x as f32 / width as f32) * 255.0) as u8,
                ((y as f32 / height as f32) * 255.0) as u8,
                255 as u8,
            ]);
        }
    }

    let texture = Texture::new(
        Extent3d {
            width,
            height,
            depth: 1,
        },
        bevy::render::texture::TextureDimension::D2,
        image.into_raw(),
        TextureFormat::Rgba8UnormSrgb,
    );

    let handle = textures.add(texture);

    let material = ColorMaterial {
        texture: Some(handle),
        ..Default::default()
    };

    commands.spawn(SpriteBundle {
        sprite: Sprite::new(Vec2::new(width as f32, height as f32)),
        material: materials.add(material),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(mandelbrot_generation_system.system())
        .run();
}

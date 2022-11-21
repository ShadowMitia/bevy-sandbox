//! Shows how to render simple primitive shapes with a single color.

use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
    sprite::{Material2d, Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_plugin(Material2dPlugin::<CustomMaterial>::default())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Rectangle
    // commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::rgb(0.25, 0.25, 0.75),
    //         custom_size: Some(Vec2::new(50.0, 100.0)),
    //         ..default()
    //     },
    //     ..default()
    // });

    let diameter = 200f32;
    let num_sections = 10;

    // Circle
    for i in 1..(num_sections + 1) {
        let r = (diameter / num_sections as f32) * ((num_sections - i + 1) as f32);

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(r).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(
                0.3,
                0.12 * (i as f32),
                (i as f32) * 0.15,
            ))),
            transform: Transform::from_translation(Vec3::new(
                -100.0 - (num_sections - i - 1) as f32 * (diameter / num_sections as f32),
                0.,
                0.,
            )),
            ..Default::default()
        });
    }

    // Based on https://twitter.com/Ayliean/status/1593276676263256068
    for i in 1..(num_sections + 1) {
        let r = (diameter / num_sections as f32) * ((num_sections - i + 1) as f32);

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(r).into()).into(),
            material: custom_materials.add(CustomMaterial {
                color: Color::rgb(0.3, 0.12 * (i as f32), (i as f32) * 0.15),
            }),
            transform: Transform::from_translation(Vec3::new(
                (num_sections - i - 1) as f32 * (diameter / num_sections as f32),
                0.,
                0.,
            )),
            ..default()
        });
    }

    // Hexagon
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
    //     material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
    //     transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
    //     ..default()
    // });
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c620"]
struct CustomMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl Material2d for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/equal_circle.vert".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/equal_circle.frag".into()
    }

    // Bevy assumes by default that vertex shaders use the "vertex" entry point
    // and fragment shaders use the "fragment" entry point (for WGSL shaders).
    // GLSL uses "main" as the entry point, so we must override the defaults here

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        key: bevy::sprite::Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}

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
};

/// This example illustrates how to load shaders such that they can be
/// edited while the example is still running.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_plugin(MaterialPlugin::<ToonMaterial>::default())
        .add_plugin(MaterialPlugin::<PhongMaterial>::default())
        .add_plugin(MaterialPlugin::<BlinnPhongMaterial>::default())
        .add_startup_system(setup_system)
        .run();
}

/*
    NOTE: Shaders and Materials are probably best used merged together with toggles
    NOTE: Such as with bevy's "shader defs"
    NOTE: This is fine for learning purposes, and having shader building blocks
    NOTE: But probably not the most efficient! (Or practical to store in assets)

*/

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c620"]
struct CustomMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/hot.vert".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/hot.frag".into()
    }

    // Bevy assumes by default that vertex shaders use the "vertex" entry point
    // and fragment shaders use the "fragment" entry point (for WGSL shaders).
    // GLSL uses "main" as the entry point, so we must override the defaults here
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c621"]
struct ToonMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl Material for ToonMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/toon.vert".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/toon.frag".into()
    }

    // Bevy assumes by default that vertex shaders use the "vertex" entry point
    // and fragment shaders use the "fragment" entry point (for WGSL shaders).
    // GLSL uses "main" as the entry point, so we must override the defaults here
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c622"]
struct PhongMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl Material for PhongMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/phong.vert".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/phong.frag".into()
    }

    // Bevy assumes by default that vertex shaders use the "vertex" entry point
    // and fragment shaders use the "fragment" entry point (for WGSL shaders).
    // GLSL uses "main" as the entry point, so we must override the defaults here
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c623"]
struct BlinnPhongMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl Material for BlinnPhongMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/blinnphong.vert".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/blinnphong.frag".into()
    }

    // Bevy assumes by default that vertex shaders use the "vertex" entry point
    // and fragment shaders use the "fragment" entry point (for WGSL shaders).
    // GLSL uses "main" as the entry point, so we must override the defaults here
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut toon_materials: ResMut<Assets<ToonMaterial>>,
    mut phong_materials: ResMut<Assets<PhongMaterial>>,
    mut blinnphong_materials: ResMut<Assets<BlinnPhongMaterial>>,
) {
    // Setup our world
    commands
        // cube
        .spawn(MaterialMeshBundle {
            material: materials.add(CustomMaterial {
                color: Color::rgb(0.0, 0.8, 0.0),
            }),
            mesh: meshes.add(Mesh::from(shape::Cube { size: 3.0 })),
            transform: Transform::from_translation(Vec3::new(-5.0, 0.0, 0.0)),
            ..Default::default()
        });
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 2.0,
            ..Default::default()
        })),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        material: toon_materials.add(ToonMaterial {
            color: Color::rgb(0.8, 0.0, 0.8),
        }),
        ..Default::default()
    });
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 2.0,
            ..Default::default()
        })),
        transform: Transform::from_translation(Vec3::new(5.0, 0.0, 0.0)),
        material: phong_materials.add(PhongMaterial {
            color: Color::rgb(0.8, 0.0, 0.3),
        }),
        ..Default::default()
    });
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 2.0,
            ..Default::default()
        })),
        transform: Transform::from_translation(Vec3::new(10.0, 1.0, 0.0)),
        material: blinnphong_materials.add(BlinnPhongMaterial {
            color: Color::rgb(0.8, 0.0, 0.3),
        }),
        ..Default::default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(3.0, 5.0, -20.0))
            .looking_at(Vec3::default(), Vec3::Y),
        ..Default::default()
    });
}

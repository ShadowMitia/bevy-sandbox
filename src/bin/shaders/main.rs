use std::path::Path;

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::ShaderStages,
    },
};

/// This example illustrates how to load shaders such that they can be
/// edited while the example is still running.
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_asset::<MyMaterial>()
        .add_asset::<ToonMaterial>()
        .add_asset::<PhongMaterial>()
        .add_asset::<BlinnPhongMaterial>()
        .add_startup_system(setup)
        .run();
}

/*
    NOTE: Shaders and Materials are probably best used merged together with toggles
    NOTE: Such as with bevy's "shader defs"
    NOTE: This is fine for learning purposes, and having shader building blocks
    NOTE: But probably not the most efficient! (Or practical to store in assets)
    
*/

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c620"]
struct MyMaterial {
    pub color: Color,
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c621"]
struct ToonMaterial {
    pub color: Color,
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c622"]
struct PhongMaterial {
    pub color: Color,
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c623"]
struct BlinnPhongMaterial {
    pub color: Color,
}

fn build_shader_pipeline<T: bevy::render::renderer::RenderResources + bevy::reflect::TypeUuid>(
    shader_name: String,
    pipelines: &mut ResMut<Assets<PipelineDescriptor>>,
    render_graph: &mut ResMut<RenderGraph>,
    asset_server: &mut ResMut<AssetServer>,
) -> Handle<PipelineDescriptor> {
    // Create a new shader pipeline with shaders loaded from the asset directory
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: asset_server.load::<Shader, _>(Path::new(&format!("shaders/{}.vert", shader_name))),
        fragment: Some(
            asset_server.load::<Shader, _>(Path::new(&format!("shaders/{}.frag", shader_name))),
        ),
    }));

    // Add an AssetRenderResourcesNode to our Render Graph. This will bind MyMaterial resources to our shader
    render_graph.add_system_node(
        format!("{}_material", shader_name),
        AssetRenderResourcesNode::<T>::new(true),
    );

    // Add a Render Graph edge connecting our new "my_material" node to the main pass node. This ensures "my_material" runs before the main pass
    render_graph
        .add_node_edge(format!("{}_material", shader_name), base::node::MAIN_PASS)
        .unwrap();

    pipeline_handle
}

fn setup(
    commands: &mut Commands,
    mut asset_server: ResMut<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MyMaterial>>,
    mut toon_materials: ResMut<Assets<ToonMaterial>>,
    mut phong_materials: ResMut<Assets<PhongMaterial>>,
    mut blinnphong_materials: ResMut<Assets<BlinnPhongMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    // Watch for changes
    asset_server.watch_for_changes().unwrap();

    let pipeline_handle = build_shader_pipeline::<MyMaterial>(
        "hot".to_string(),
        &mut pipelines,
        &mut render_graph,
        &mut asset_server,
    );

    let toon_pipeline_handle = build_shader_pipeline::<ToonMaterial>(
        "toon".to_string(),
        &mut pipelines,
        &mut render_graph,
        &mut asset_server,
    );

    let phong_pipeline_handle = build_shader_pipeline::<PhongMaterial>(
        "phong".to_string(),
        &mut pipelines,
        &mut render_graph,
        &mut asset_server,
    );

    let blinnphong_pipeline_handle = build_shader_pipeline::<BlinnPhongMaterial>(
        "blinnphong".to_string(),
        &mut pipelines,
        &mut render_graph,
        &mut asset_server,
    );

    let material = materials.add(MyMaterial {
        color: Color::rgb(0.0, 0.8, 0.0),
    });

    // Create a new material
    let toon_material = toon_materials.add(ToonMaterial {
        color: Color::rgb(0.8, 0.0, 0.8),
    });

    // Create a new material
    let phong_material = phong_materials.add(PhongMaterial {
        color: Color::rgb(0.8, 0.0, 0.3),
    });

    // Create a new material
    let blinnphong_material = blinnphong_materials.add(BlinnPhongMaterial {
        color: Color::rgb(0.8, 0.0, 0.3),
    });

    // Setup our world
    commands
        // cube
        .spawn(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 3.0 })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_translation(Vec3::new(-5.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(material)
        .spawn(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 2.0,
                ..Default::default()
            })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                toon_pipeline_handle,
            )]),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(toon_material)
        .spawn(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 2.0,
                ..Default::default()
            })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                phong_pipeline_handle,
            )]),
            transform: Transform::from_translation(Vec3::new(5.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(phong_material)
        .spawn(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 2.0,
                ..Default::default()
            })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                blinnphong_pipeline_handle,
            )]),
            transform: Transform::from_translation(Vec3::new(10.0, 1.0, 0.0)),
            ..Default::default()
        })
        .with(blinnphong_material)
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(3.0, 5.0, -20.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
}

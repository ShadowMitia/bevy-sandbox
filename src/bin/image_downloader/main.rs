use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    utils::BoxedFuture,
};

use futures::future::join_all;
use rand::seq::SliceRandom;
use serde::Deserialize;

#[derive(Component)]
struct WebImage;

#[derive(Resource)]
struct RedditImages {
    images: Vec<Image>,
}

struct RedditHandles {
    handles: Vec<Handle<Image>>,
}

fn setup_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    links: Res<RedditImages>,
    mut textures: ResMut<Assets<Image>>,
) {
    commands.spawn(Camera2dBundle::default());

    let window = windows.get_primary_mut().unwrap();
    window.set_title("Reddit images".to_string());

    let window_width = window.width();
    let window_height = window.height();

    let size = 256.0;

    let width = window_width as f32 / size;
    let height = window_height as f32 / size;

    let half_width = (window_width as f32 + 0.5) / 2.0;
    let half_height = (window_height as f32 + 0.5) / 2.0;

    let texs = &links.images;

    let mut handles = Vec::new();

    for tex in texs {
        let texture_handle: Handle<Image> = textures.add(tex.clone());
        handles.push(texture_handle);
    }

    let images = RedditHandles { handles };

    let images = &images.handles;
    let mut mats = Vec::new();
    for image in images {
        let mat = image.clone();
        mats.push(mat);
    }

    for j in 0..(height as u32) {
        for i in 0..(width as u32) {
            // let zero_or_one = if rand::random() { 1.0 } else { 0.0 };
            let mat = images.choose(&mut rand::thread_rng()).unwrap().clone();
            commands.spawn((
                SpriteBundle {
                    texture: mat.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        half_width - size * i as f32 - size / 2.0,
                        half_height - size * j as f32 - size / 2.0,
                        0.0,
                    )),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(size, size)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                WebImage,
            ));
        }
    }
}

// struct UpdateTimer(Timer);

// fn update_sprites(
//     time: Res<Time>,
//     mut timer: ResMut<UpdateTimer>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut query: Query<(&Image, &mut SpriteComponents)>,
// ) {
//     timer.0.tick(time.delta_seconds);

//     if timer.0.finished {
//         for (_, mut sprite) in query.iter_mut() {}
//     }
// }

async fn get_texture_from_url(url: &str) -> Option<Image> {
    println!("getting {}", url);
    let bytes = reqwest::get(url).await.unwrap().bytes().await.unwrap();
    let image = match image::load_from_memory(&bytes) {
        Ok(image) => image.to_rgba8(),
        Err(_) => return None,
    };

    let width = image.width();
    let height = image.height();
    let data = image.into_vec();
    Some(Image::new_fill(
        Extent3d {
            width,
            height,
            ..Default::default()
        },
        TextureDimension::D2,
        &data,
        TextureFormat::Rgba8Unorm,
    ))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subbreddit = roux::Subreddit::new("aww");

    let num_images = 20;

    let articles = subbreddit
        .hot(num_images, None)
        .await
        .unwrap()
        .data
        .children;

    let links = articles
        .iter()
        .filter(|a| a.kind == "t3")
        .map(|a| a.data.url.clone().unwrap())
        .filter(|url| url.split('.').last().is_some())
        .collect::<Vec<String>>();

    let mut tasks = Vec::new();

    for url in links.iter() {
        let url = url.clone();
        tasks.push(tokio::spawn(
            async move { get_texture_from_url(&url).await },
        ));
    }

    println!("joining...");
    let items = join_all(tasks).await;
    println!("done!");
    let items = items.into_iter().filter(|a| a.is_ok());

    let mut images = Vec::new();

    for image in items {
        let res = match image {
            Ok(res) => res,
            Err(_) => continue,
        };
        let res = match res {
            Some(t) => t,
            None => continue,
        };
        images.push(res);
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_asset::<JpegAsset>()
        .init_asset_loader::<JpegAssetLoader>()
        .insert_resource(RedditImages { images })
        // .add_resource(UpdateTimer(Timer::from_seconds(1.0, true)))
        .add_startup_system(setup_system)
        // .add_system(update_sprites.system())
        .run();

    Ok(())
}

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct JpegAsset;

#[derive(Default)]
pub struct JpegAssetLoader;

impl AssetLoader for JpegAssetLoader {
    fn load<'a>(
        &'a self,

        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let dyn_img =
                image::load_from_memory_with_format(bytes, image::ImageFormat::Jpeg)?.to_rgba8();

            let width = dyn_img.width();
            let height = dyn_img.height();
            let data = dyn_img.to_vec();

            let custom_asset = Image::new_fill(
                Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                &data,
                TextureFormat::Rgba8Unorm,
            );
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["jpeg", "jpg"]
    }
}

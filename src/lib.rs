use bevy::{asset::HandleId, prelude::*, render::texture::{ImageType, CompressedImageFormats}};

pub struct DebugTexturePlugin;
impl Plugin for DebugTexturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_texture);
        app.add_systems(PostUpdate, replace_blank_textures);
    }
}

#[derive(Resource, Deref, DerefMut, Clone)]
pub struct DebugMaterial(pub Handle<StandardMaterial>);

pub fn setup_texture(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let raw_texture = include_bytes!("texture.png");

    let image = Image::from_buffer(
        raw_texture,
        ImageType::Extension("png"),
        CompressedImageFormats::BC,
        false,
    )
    .expect("Could not create debug texture");

    let material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(image)),
        perceptual_roughness: 0.95,
        reflectance: 0.05,
        ..default()
    });

    commands.insert_resource(DebugMaterial(material));
}

pub fn replace_blank_textures(
    mut materials: Query<&mut Handle<StandardMaterial>, Changed<Handle<StandardMaterial>>>,
    debug_material: Res<DebugMaterial>,
) {
    for mut handle in &mut materials {
        match handle.id() {
            HandleId::Id(_, id) if id == 0 => {
                *handle = debug_material.0.clone();
            }
            _ => {}
        }
    }
}

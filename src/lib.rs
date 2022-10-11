use std::num::NonZeroU8;
use bevy::asset::AssetLoader;
use bevy::asset::LoadContext;
use bevy::asset::LoadedAsset;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::AddressMode;
use bevy::render::render_resource::CompareFunction;
use bevy::render::render_resource::FilterMode;
use bevy::render::render_resource::SamplerDescriptor;
use bevy::render::texture::ImageSampler;
use bevy::utils::BoxedFuture;
use bimap::BiMap;
use serde::Deserialize;
use wgpu::SamplerBorderColor;

const IMG_CFG_EXTENSION: &'static str = "img_cfg";

#[derive(Debug, Clone, TypeUuid, Deserialize)]
#[uuid = "3434ddaa-eead-327d-db95-4ccc87522222"]
struct ImageSamplerConfig {
    #[serde(default)]
    pub address_mode_u: AddressMode,
    #[serde(default)]
    pub address_mode_v: AddressMode,
    #[serde(default)]
    pub address_mode_w: AddressMode,
    #[serde(default)]
    pub mag_filter: FilterMode,
    #[serde(default)]
    pub min_filter: FilterMode,
    #[serde(default)]
    pub mipmap_filter: FilterMode,
    #[serde(default)]
    pub lod_min_clamp: f32,
    #[serde(default)]
    pub lod_max_clamp: f32,
    #[serde(default)]
    pub compare: Option<CompareFunction>,
    #[serde(default)]
    pub anisotropy_clamp: Option<NonZeroU8>,
    #[serde(default)]
    pub border_color: Option<SamplerBorderColor>,
}

impl ImageSamplerConfig {
    fn get_sampler_descriptor(&self) -> ImageSampler {
        let desc = SamplerDescriptor {
            label: None,
            address_mode_u: self.address_mode_u,
            address_mode_v: self.address_mode_v,
            address_mode_w: self.address_mode_w,
            mag_filter: self.mag_filter, 
            min_filter: self.min_filter,
            mipmap_filter: self.mipmap_filter,
            lod_min_clamp: self.lod_min_clamp,
            lod_max_clamp: self.lod_max_clamp,
            compare: self.compare,
            anisotropy_clamp: self.anisotropy_clamp,
            border_color: self.border_color,
        };
        ImageSampler::Descriptor(desc)
    }
}


#[derive(Default, Deref, DerefMut)]
struct ImageConfigs(BiMap<Handle<Image>, Handle<ImageSamplerConfig>>);

#[derive(Default)]
struct ImageConfigLoader;

impl AssetLoader for ImageConfigLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let config: ImageSamplerConfig = ron::de::from_bytes(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(config));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &[IMG_CFG_EXTENSION]
    }
}

fn config_image(
    asset_server: Res<AssetServer>,
    mut image_asset_events: EventReader<AssetEvent<Image>>,
    mut image_config_asset_events: EventReader<AssetEvent<ImageSamplerConfig>>,
    mut image_assets: ResMut<Assets<Image>>,
    config_assets: Res<Assets<ImageSamplerConfig>>,
    mut image_configs: ResMut<ImageConfigs>,
) {
    for event in image_asset_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(path) = asset_server.get_handle_path(handle) {
                    let path = path.path();
                    let image_cfg_path = path.with_extension(IMG_CFG_EXTENSION);
                    if asset_server.asset_io().is_file(&image_cfg_path) {
                        let image_cfg: Handle<ImageSamplerConfig> = asset_server.load(image_cfg_path);
                        image_configs.insert(handle.clone_weak(), image_cfg);
                    } 
                }
            },
            AssetEvent::Modified { .. } => {
                
            },
            AssetEvent::Removed { handle } => {
                image_configs.remove_by_left(handle);
            },
        }
    }

    for event in image_config_asset_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(image_handle) = image_configs.get_by_right(handle) {
                    if let Some(image) = image_assets.get_mut(image_handle) {
                        let image_config = config_assets.get(handle).unwrap();
                        image.sampler_descriptor = image_config.get_sampler_descriptor();
                    }
                }
            },
            AssetEvent::Modified { handle } => {
                if let Some(image_handle) = image_configs.get_by_right(handle) {
                    if let Some(image) = image_assets.get_mut(image_handle) {
                        let image_config = config_assets.get(handle).unwrap();
                        image.sampler_descriptor = image_config.get_sampler_descriptor();
                    }
                }
            },
            AssetEvent::Removed { handle } => {
                image_configs.remove_by_right(handle);
            },
        }
    }
}

pub struct ImageConfigPlugin;

impl Plugin for ImageConfigPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_asset::<ImageSamplerConfig>()
        .init_asset_loader::<ImageConfigLoader>()
        .init_resource::<ImageConfigs>()
        .add_system(config_image);
    }
}
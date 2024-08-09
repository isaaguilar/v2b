use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, AsyncReadExt, LoadContext};
use bevy::prelude::TypePath;
use bevy::prelude::*;
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Default)]
pub struct SvgAsset {
    pub points: Vec<Vec2>,
}

impl SvgAsset {
    fn new(svg_buf: Vec<u8>) -> Self {
        let points = crate::from_path(svg_buf).unwrap(); // todo err
        Self { points }
    }
}

#[derive(Default)]
pub struct SvgAssetLoader;

/// Possible errors that can be produced by [`SvgAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum SvgAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetLoader for SvgAssetLoader {
    type Asset = SvgAsset;
    type Settings = ();
    type Error = SvgAssetLoaderError;
    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let svg_asset = SvgAsset::new(bytes);

        Ok(svg_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["svg"]
    }
}

pub struct SvgPlugin;

impl Plugin for SvgPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SvgAsset>()
            .init_asset_loader::<SvgAssetLoader>();
    }
}

/// A Bevy `Bundle` to represent a shape.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct PathBundle {
    pub handler: Handle<SvgAsset>,
}

impl Default for PathBundle {
    fn default() -> Self {
        Self { handler: default() }
    }
}

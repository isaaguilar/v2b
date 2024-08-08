use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, AsyncReadExt, LoadContext};
use bevy::prelude::TypePath;
use thiserror::Error;

#[derive(Asset, TypePath, Debug)]
pub struct SvgAsset {
    pub points: Vec<(f32, f32)>,
}

impl SvgAsset {
    fn new(svg_buf: Vec<u8>) -> Self {
        let points = crate::get_paths(svg_buf).unwrap(); // todo err
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

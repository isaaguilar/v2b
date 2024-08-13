use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, AsyncReadExt, LoadContext};
use bevy::prelude::TypePath;
use bevy::prelude::*;
use thiserror::Error;
use usvg::Tree;

#[derive(Asset, TypePath, Debug, Default)]
pub struct SvgAsset {
    pub paths: Vec<SvgPath>,
    pub dimensions: Vec2,
}

#[derive(Debug, Default)]
pub struct SvgPath {
    pub points: Vec<Vec2>,
    pub translation: Vec2,
}

impl SvgAsset {
    fn new(svg_buf: Vec<u8>) -> Self {
        let mut paths = vec![];

        let tree = Tree::from_data(
            &svg_buf,
            &usvg::Options {
                dpi: 96.0,
                ..default()
            },
        )
        .unwrap(); // TODO err
        debug!("{:#?}", tree);

        let usvg_paths = crate::collect_paths_in_nodes(tree.root());
        debug!(?usvg_paths);

        for usvg_path in usvg_paths {
            let mut buf: Vec<Vec2> = vec![];
            crate::write_points(
                usvg_path.data(),
                &mut buf,
                usvg_path.abs_transform(),
                usvg_path.abs_bounding_box(),
            );

            paths.push(SvgPath {
                points: buf,
                translation: Vec2::new(
                    usvg_path.abs_bounding_box().left(),
                    usvg_path.abs_bounding_box().top(),
                ),
            });
        }

        let dimensions = Vec2::new(tree.size().width(), tree.size().height());

        Self {
            paths: paths,
            dimensions: dimensions,
        }
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
    pub texture: Handle<Image>,
    pub transform: Transform,
}

impl Default for PathBundle {
    fn default() -> Self {
        Self {
            handler: default(),
            texture: default(),
            transform: default(),
        }
    }
}

impl PathBundle {
    pub fn new() -> Self {
        Self {
            handler: default(),
            texture: default(),
            transform: default(),
        }
    }
}

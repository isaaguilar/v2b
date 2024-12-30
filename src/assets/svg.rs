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

#[derive(Component, Debug, Default)]
pub struct SvgAssetHandle(pub Handle<SvgAsset>);

#[derive(Component, Debug, Default)]
pub struct ImageHandle(pub Handle<Image>);

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
    #[error("Could not load asset:: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetLoader for SvgAssetLoader {
    type Asset = SvgAsset;
    type Settings = ();
    type Error = SvgAssetLoaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
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

/// A Bevy `Bundle` to represent a shape with a texture.
#[derive(Bundle)]
pub struct SvgTextureBundle {
    pub svg_handle: SvgAssetHandle,
    pub texture: ImageHandle,
    pub transform: Transform,
}

impl Default for SvgTextureBundle {
    fn default() -> Self {
        Self {
            svg_handle: default(),
            texture: default(),
            transform: default(),
        }
    }
}

/// A Bevy `Bundle` to represent a shape.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct SvgFile {
    pub handle: SvgAssetHandle,
}

impl Default for SvgFile {
    fn default() -> Self {
        Self { handle: default() }
    }
}

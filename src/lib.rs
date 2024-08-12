use bevy::prelude::*;
use lyon_geom::{cubic_bezier::CubicBezierSegment, quadratic_bezier::QuadraticBezierSegment};
use std::{fs::File, io::Read};
use usvg::{tiny_skia_path::PathSegment, Node, Path, Tree};

pub mod assets;
pub use assets::svg::{PathBundle, SvgAsset, SvgAssetLoader, SvgPlugin};

const TOLERANCE: f32 = 0.25;

fn write_points(
    path: &usvg::tiny_skia_path::Path,
    buf: &mut Vec<Vec2>,
    abs_transform: usvg::Transform,
    abs_bounding_box: usvg::Rect,
) {
    let mut last = (0.0, 0.0);
    let scale_x = abs_transform.sx;
    let scale_y = abs_transform.sy;
    let transform_x = abs_transform.tx - abs_bounding_box.left();
    let transform_y = abs_transform.ty - abs_bounding_box.top();

    for seg in path.segments() {
        match seg {
            PathSegment::MoveTo(pt) => {
                buf.push(Vec2::new(
                    (pt.x * scale_x) + transform_x,
                    -((pt.y * scale_y) + transform_y),
                ));
                last = (pt.x, pt.y);
            }
            PathSegment::LineTo(pt) => {
                buf.push(Vec2::new(
                    (pt.x * scale_x) + transform_x,
                    -((pt.y * scale_y) + transform_y),
                ));
                last = (pt.x, pt.y);
            }
            PathSegment::QuadTo(p1, p) => {
                let q = QuadraticBezierSegment {
                    from: last.into(),
                    ctrl: (p1.x, p1.y).into(),
                    to: (p.x, p.y).into(),
                };
                for pt in q.flattened(TOLERANCE) {
                    buf.push(Vec2::new(
                        (pt.x * scale_x) + transform_x,
                        -((pt.y * scale_y) + transform_y),
                    ));
                }
                last = (p.x, p.y);
            }
            PathSegment::CubicTo(p1, p2, p) => {
                let c = CubicBezierSegment {
                    from: last.into(),
                    ctrl1: (p1.x, p1.y).into(),
                    ctrl2: (p2.x, p2.y).into(),
                    to: (p.x, p.y).into(),
                };
                for pt in c.flattened(TOLERANCE) {
                    buf.push(Vec2::new(
                        (pt.x * scale_x) + transform_x,
                        -((pt.y * scale_y) + transform_y),
                    ));
                }
                last = (p.x, p.y);
            }
            PathSegment::Close => {}
        }
    }
}

fn collect_paths_in_nodes(parent: &usvg::Group) -> Vec<Path> {
    parent
        .children()
        .iter()
        .fold::<Vec<Path>, _>(vec![], |mut paths, node| {
            match node {
                Node::Group(g) => {
                    let mut p = collect_paths_in_nodes(g);
                    paths.append(&mut p);
                }
                Node::Path(p) => paths.push(*p.clone()),
                Node::Image(_) => {}
                Node::Text(_) => {}
            }
            paths
        })
}

pub fn load(
    filename: String,
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut svg_buf = Vec::default();
    let _ = File::open(filename)?.read_to_end(&mut svg_buf)?;
    Ok(svg_buf)
}

pub fn from_path(
    svg_buf: Vec<u8>,
) -> Result<Vec<Vec2>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let tree = Tree::from_data(&svg_buf, &usvg::Options::default())?;
    debug!(?tree);

    let paths = collect_paths_in_nodes(tree.root());
    debug!(?paths);

    let mut buf: Vec<Vec2> = vec![];
    for path in paths {
        write_points(
            path.data(),
            &mut buf,
            path.abs_transform(),
            path.abs_bounding_box(),
        );
    }

    Ok(buf)
}

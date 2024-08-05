use lyon_geom::{cubic_bezier::CubicBezierSegment, quadratic_bezier::QuadraticBezierSegment};
use std::{fs::File, io::Read};
use tracing::debug;
use usvg::{tiny_skia_path::PathSegment, Node, Path, Tree};

const TOLERANCE: f32 = 0.25;

fn write_path(path: &usvg::tiny_skia_path::Path, buf: &mut Vec<(f32, f32)>, normalize: f32) {
    let mut last = (0.0, 0.0);
    for seg in path.segments() {
        match seg {
            PathSegment::MoveTo(p) => {
                buf.push((p.x, normalize - p.y));
                last = (p.x, p.y);
            }
            PathSegment::LineTo(p) => {
                buf.push((p.x, normalize - p.y));
                last = (p.x, p.y);
            }
            PathSegment::QuadTo(p1, p) => {
                let q = QuadraticBezierSegment {
                    from: last.into(),
                    ctrl: (p1.x, p1.y).into(),
                    to: (p.x, p.y).into(),
                };
                for pt in q.flattened(TOLERANCE) {
                    buf.push((pt.x, normalize - pt.y));
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
                    buf.push((pt.x, normalize - pt.y));
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

pub fn get_paths(
    filename: String,
) -> Result<Vec<(f32, f32)>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut buf: Vec<(f32, f32)> = vec![];
    let mut svg_buf = Vec::default();
    let _ = File::open(filename)?.read_to_end(&mut svg_buf)?;

    let tree = Tree::from_data(&svg_buf, &usvg::Options::default())?;
    debug!(?tree);
    let raw_height = tree.size().height();

    let paths = collect_paths_in_nodes(tree.root());
    debug!(?paths);

    for path in paths {
        write_path(path.data(), &mut buf, raw_height / 1.3333313);
    }

    Ok(buf)
}

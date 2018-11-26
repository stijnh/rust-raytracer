use std::fs::File;
use std::io::{BufReader, BufRead, Result as IoResult, Error as IoError};
use json::{JsonValue, Error as JsonError, parse as json_parse};
use geom::{Geometry, GeometryList, Triangle};
use math::{Vec3D, vec3d};
use camera::Camera;

#[derive(Debug, Fail)]
pub enum LoaderError {
    #[fail(display="{}", 0)]
    IoError(IoError),

    #[fail(display="{}", 0)]
    ParseError(String),

    #[fail(display="{}", 0)]
    JsonError(JsonError),
}

pub fn load_obj(filename: &str) -> Result<Vec<(Vec3D, Vec3D, Vec3D)>, LoaderError> {
    let mut vertices = vec![];
    let mut triangles = vec![];

    let parse_error = |msg, lineno| {
        Err(LoaderError::ParseError(format!(
               "{}: line {} of file '{}'",
               msg,
               lineno + 1,
               filename)))
    };

    let file = File::open(filename).map_err(|e| LoaderError::IoError(e))?;

    for (lineno, line) in BufReader::new(file).lines().enumerate() {
        let line = line.map_err(|e| LoaderError::IoError(e))?;
        let mut iter = line.trim().split_whitespace();

        let token = match iter.next() {
            Some(c) if !c.starts_with('#') => c,
            _ => continue,
        };

        match token {
            "v" => {
                let xs = iter.next().and_then(|p| p.parse().ok());
                let ys = iter.next().and_then(|p| p.parse().ok()); 
                let zs = iter.next().and_then(|p| p.parse().ok());

                if let (Some(x), Some(y), Some(z)) = (xs, ys, zs) {
                    vertices.push(vec3d(x, y, z));
                } else {
                    return parse_error("failed to parse vertex".into(), lineno);
                }
            },
            "f" => {
                let is = iter.next().and_then(|p| p.parse::<usize>().ok());
                let js = iter.next().and_then(|p| p.parse::<usize>().ok());
                let ks = iter.next().and_then(|p| p.parse::<usize>().ok());

                if let (Some(i), Some(j), Some(k)) = (is, js, ks) {
                    if min!(i, j, k) == 0 || max!(i, j, k) < vertices.len() + 1 {
                        triangles.push((
                            vertices[i - 1], 
                            vertices[j - 1], 
                            vertices[k - 1]));

                    } else {
                        return parse_error(format!(
                            "vertex {} is invalid, {} vertices found",
                            max!(i, j, k),
                            vertices.len()),
                            lineno);
                    }
                } else {
                    return parse_error("failed to parse face".into(), lineno);
                }

            },
            "vt" => {
                /* TODO */ 
            },
            "vn" => {
                /* TODO */    
            },
            _ => {
                return parse_error(format!("unexpected token '{}'", token), 
                                   lineno);
            }
        }

    }

    Ok(triangles)
}

pub fn load_scene(filename: &str) -> Result<(Camera, Box<dyn Geometry>), LoaderError> {
    let content = ::std::fs::read_to_string(filename).map_err(|e| LoaderError::IoError(e))?;
    let root = json_parse(&content).map_err(|e| LoaderError::JsonError(e))?;

    let parse_error = |msg, path| {
        LoaderError::ParseError(format!(
            "{}: in file {} for {}", msg, filename, path))
    };

    let parse_vec3d = |val: &JsonValue, path| {
        let (x, y, z) = if val.is_array() {
            (&val[0], &val[1], &val[2])
        } else if val.is_object() {
            (&val["x"], &val["y"], &val["z"])
        } else if val.is_null() {
            raise!(parse_error(format!(
                "element must be array or object, not {}",
                val), path));
        } else {
            raise!(parse_error("element not found".into(), path));
        };

        match (x.as_f32(), y.as_f32(), z.as_f32()) {
            (Some(x), Some(y), Some(z)) => {
                Ok(vec3d(x, y, z))
            },
            _ => {
                raise!(parse_error(format!("failed to parse ({}, {}, {}) as 3D vector",
                    x, y, z), path));
            }
        }
    };

    let cam_pos = parse_vec3d(&root["camera"]["position"], "camera.position")?;
    let cam_lookat = parse_vec3d(&root["camera"]["lookat"], "camera.lookat")?;
    let cam_up = parse_vec3d(&root["camera"]["up"], "camera.up")?;

    let cam = Camera::new()
        .position(cam_pos)
        .look_at(cam_lookat, cam_up);

    let list: GeometryList<_> = GeometryList::<Triangle>::from_vec(vec![]);
    let out: Box<GeometryList<_>> = Box::new(list);

    Ok((cam, out))
}

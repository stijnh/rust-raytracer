use std::fs::File;
use std::io::{BufReader, BufRead, Result as IoResult, Error as IoError};
use object::Triangle;
use vec3::{Vec3, vec3};

#[derive(Debug, Fail)]
pub enum LoaderError {
    #[fail(display="{}", 0)]
    IoError(IoError),

    #[fail(display="{}", 0)]
    ParseError(String),
}

pub fn load_obj(filename: &str) -> Result<Vec<(Vec3, Vec3, Vec3)>, LoaderError> {
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
                    vertices.push(vec3(x, y, z));
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

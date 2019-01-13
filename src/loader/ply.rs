use crate::geom::Mesh;
use crate::math::Vec3D;
use failure::Fail;
use std::fs::read_to_string;
use std::io;

#[derive(Fail, Debug)]
pub enum LoadError {
    #[fail(display = "Error while reading file")]
    IO(#[cause] io::Error),

    #[fail(display = "File not in PLY format")]
    Format,

    #[fail(display = "Parse error at line {}: {}", _0, _1)]
    Parse(usize, String),

    #[fail(display = "Invalid face {}", _0)]
    InvalidFace(String),
}

fn normalize_type(parts: &[&str]) -> Option<String> {
    if let [typ] = parts {
        Some(match *typ {
            "char" | "int8" => "int8",
            "uchar" | "uint8" => "uint8",
            "short" | "int16" => "int16",
            "ushort" | "uint16" => "uint16",
            "int" | "int32" => "int32",
            "uint" | "uint32" => "uint32",
            "float" | "float32" => "float32",
            "double" | "float64" => "float64",
            _ => return None,
        }.to_string())
    } else if let ["list", a, b] = parts {
        Some(format!("list {} {}",
                normalize_type(&[a])?,
                normalize_type(&[b])?))
    } else {
        None
    }
}

struct Reader<'a> {
    lineno: usize,
    buffer: &'a str,
    current: Vec<&'a str>,
}

impl<'a> Reader<'a> {
    fn new(buffer: &'a str) -> Self {
        Self {
            lineno: 0,
            buffer,
            current: vec![],
        }
    }

    fn next(&mut self) -> &[&'a str] {
        self.current.clear();

        if let Some(i) = self.buffer.find('\n') {
            self.current.extend(self.buffer[..i].split_whitespace());
            self.buffer = &self.buffer[i + 1..];
            self.lineno += 1;
        } else {
            self.buffer = &"";
        }

        self.current()
    }

    fn current(&self) -> &[&'a str] {
        &*self.current
    }
}

type Segment = (String, usize, Vec<(String, String)>);

fn parse_err(reader: &Reader, msg: &str) -> LoadError {
    LoadError::Parse(reader.lineno, msg.to_string())
}

fn parse_header<'a>(lines: &mut Reader<'a>) -> Result<Vec<Segment>, LoadError> {
    if lines.next() != &["ply"] {
        raise!(parse_err(lines, "expecting 'ply'"));
    }

    if lines.next() != &["format", "ascii", "1.0"] {
        raise!(parse_err(lines, "expected 'format ascii 1.0'"));
    }

    lines.next();
    let mut segments = vec![];

    loop {
        let line = lines.current();

        match line.get(0) {
            Some(&"element") => (),
            Some(&"comment") => {
                lines.next();
                continue;
            }
            _ => break,
        }

        let name = line.get(1).unwrap().to_string();
        let num = line
            .get(2)
            .unwrap_or(&&"")
            .parse::<usize>()
            .map_err(|_| parse_err(lines, "failed to parse number"))?;

        let mut props = vec![];

        loop {
            let line = lines.next();
            let rest = match line.split_first() {
                Some((&"property", rest)) => rest,
                Some((&"comment", _)) => continue,
                _ => break,
            };

            let (key, typ) = match rest.split_last() {
                Some((k, r)) => (*k, r),
                None => ("", &[] as _),
            };

            let typ = normalize_type(typ)
                .ok_or_else(|| parse_err(lines, "invalid type"))?;

            props.push((key.to_string(), typ));
        }

        segments.push((name, num, props));
    }

    if lines.current() != &["end_header"] {
        raise!(parse_err(lines, "expected 'end_header'"));
    }

    Ok(segments)
}

fn parse_vertices(
    lines: &mut Reader,
    size: usize,
    props: &[(String, String)],
) -> Result<Vec<Vec3D>, LoadError> {
    const INVALID: usize = !0;
    let [mut xi, mut yi, mut zi] = [INVALID; 3];
    let n = props.len();
    let mut vertices = vec![];

    for (index, (k, v)) in props.iter().enumerate() {
        match (k.as_str(), v.as_str()) {
            ("x", "float32") => xi = index,
            ("y", "float32") => yi = index,
            ("z", "float32") => zi = index,
            (x, _) => {
                eprintln!("WARN: ignoring vertex property {:?}", x);
            }
        }
    }

    match (xi, yi, zi) {
        (INVALID, _, _) => raise!(parse_err(lines, "vertex has no x property")),
        (_, INVALID, _) => raise!(parse_err(lines, "vertex has no y property")),
        (_, _, INVALID) => raise!(parse_err(lines, "vertex has no z property")),
        _ => (),
    }

    for _ in 0..size {
        let line = lines.next();

        if line.len() != n {
            raise!(parse_err(lines, &format!("expected {} elements", n)));
        }

        let [x, y, z] = match (
            line[xi].parse::<f32>().ok(),
            line[yi].parse::<f32>().ok(),
            line[zi].parse::<f32>().ok(),
        ) {
            (Some(x), Some(y), Some(z)) => [x, y, z],
            (None, _, _) => raise!(parse_err(lines, "failed to parse x coordinate")),
            (_, None, _) => raise!(parse_err(lines, "failed to parse y coordinate")),
            (_, _, None) => raise!(parse_err(lines, "failed to parse z coordinate")),
        };

        vertices.push(Vec3D::new(x, y, z));
    }

    Ok(vertices)
}

fn parse_faces(
    lines: &mut Reader,
    size: usize,
    props: &[(String, String)],
    num_vertices: usize,
) -> Result<Vec<[u32; 3]>, LoadError> {
    let index = props
        .iter()
        .position(|(a, b)| {
            (a == "vertex_indices" || a == "vertex_index") &&
                (b == "list uint8 int32")
        })
        .ok_or_else(|| parse_err(lines, "face has not vertex_indices property"))?;

    let mut faces = vec![];
    let mut indices = vec![];

    for _ in 0..size {
        lines.next();
        let line = lines.current();
        let n = line
            .get(index)
            .and_then(|s| s.parse::<usize>().ok())
            .ok_or_else(|| parse_err(lines, "failed to parse integer"))?;

        if n < 3 || index + n >= line.len() {
            raise!(parse_err(lines, "invalid number of vertices"));
        }

        indices.clear();
        for part in &line[index + 1..index + 1 + n] {
            let x = part
                .parse::<u32>()
                .map_err(|_| parse_err(lines, "failed to parse integer"))?;

            if x as usize >= num_vertices {
                raise!(parse_err(
                    lines,
                    &format!(
                        "invalid vertex identifier {}, only {} vertices",
                        x, num_vertices
                    )
                ));
            }

            indices.push(x);
        }

        for i in 2..indices.len() {
            faces.push([indices[0], indices[i - 1], indices[i]]);
        }
    }

    Ok(faces)
}

pub fn load_ply(file: &str) -> Result<(Vec<Vec3D>, Vec<[u32; 3]>), LoadError> {
    let buffer = read_to_string(file).map_err(LoadError::IO)?;
    let mut reader = Reader::new(&buffer);

    let segments = parse_header(&mut reader)?;
    let mut vertices = vec![];
    let mut faces = vec![];

    for (name, size, props) in segments {
        if name == "vertex" {
            vertices.extend(parse_vertices(&mut reader, size, &props)?);
        } else if name == "face" {
            faces.extend(parse_faces(&mut reader, size, &props, vertices.len())?);
        } else {
            eprintln!("WARN: ignoring element {:?}", name);
        }
    }

    if !reader.buffer.trim().is_empty() {
        eprintln!("WARN: file not read entirely");
    }

    Ok((vertices, faces))
}

pub fn load_ply_as_mesh(file: &str) -> Result<Mesh, LoadError> {
    let (vertices, faces) = load_ply(file)?;
    Ok(Mesh::from_vertices(vertices, faces))
}

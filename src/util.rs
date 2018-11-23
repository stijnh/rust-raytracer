pub use vecmat::vec::{Vec2, Vec3, Dot};
pub use vecmat::mat::{Mat2, Mat3};

#[macro_export]
macro_rules! raise {
    ($x:expr) => {
        ::std::result::Result::Err($x)?;
        panic!("unreachable code");
    };
    ($x:expr, $msg:expr) => {
        let msg = $msg.into();
        raise!($x(msg));
    };
    ($x:expr, $format:expr, $( $arg:expr),* ) => {
        raise!($x, format!($format, $($arg),*))
    };
}

#[macro_export]
macro_rules! unwrap_raise {
    ($v:expr, $($x:tt)*) => {
        match $v.into_result() {
            Ok(x) => x,
            _ => raise!($($x)*)
        }
    };
}

#[macro_export]
macro_rules! max {
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {
        match ($a, $b) {
            (a, b) => if a > b { a } else { b }
        }
    };
    ($a:expr, $($b:expr),+) => {
        max!($a, max!($($b),*))
    };
}

#[macro_export]
macro_rules! min {
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {
        match ($a, $b) {
            (a, b) => if a < b { a } else { b }
        }
    };
    ($a:expr, $($b:expr),+) => {
        min!($a, min!($($b),*))
    };
}

#[macro_export]
macro_rules! iff {
    ($a:expr, $b:expr, $c:expr) => {
        if $a { $b } else { $c }
    }
}

pub type Vec3D = Vec3<f32>;
pub type Mat3D = Mat3<f32>;

pub fn vec3d(x: f32, y: f32, z: f32) -> Vec3D {
    Vec3D::from(x, y, z)
}

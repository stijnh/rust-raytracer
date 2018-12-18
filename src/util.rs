use math::Vec3;

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
        if $a {
            $b
        } else {
            $c
        }
    };
}

pub type Color = Vec3<f32>;

pub const COLOR_WHITE: Color = Color::new(1.0, 1.0, 1.0);
pub const COLOR_BLACK: Color = Color::new(0.0, 0.0, 0.0);
pub const COLOR_RED: Color = Color::new(1.0, 0.0, 0.0);
pub const COLOR_GREEN: Color = Color::new(0.0, 1.0, 0.0);
pub const COLOR_BLUE: Color = Color::new(0.0, 0.0, 1.0);

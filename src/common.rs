#[macro_export]
macro_rules! raise {
    ($x:expr) => {
        ::std::result::Result::Err($x)?;
        panic!("unreachable code");
    };
    ($x:expr, $msg:expr) => {
        match $msg.into() {
            msg => raise!($x(msg)),
        }
    };
}

#[macro_export]
macro_rules! iff {
    ($a:expr, $b:expr, $c:expr) => {
        match ($a, $b, $c) {
            (a, b, c) => {
                if a {
                    b
                } else {
                    c
                }
            }
        }
    };
}

#[macro_export]
macro_rules! max {
    ($a:expr) => {
        $a
    };
    ($a:expr, $($b:expr),*) => {
        match ($a, max!($($b),*)) {
            (a, b) => if a < b { a } else { b }
        }
    }
}

#[macro_export]
macro_rules! min {
    ($a:expr) => {
        $a
    };
    ($a:expr, $($b:expr),*) => {
        match ($a, min!($($b),*)) {
            (a, b) => if a < b { a } else { b }
        }
    }
}

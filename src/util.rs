#[macro_export]
macro_rules! raise {
    ($x:expr) => {
        {
            ::std::result::Result::Err($x)?;
            panic!("unreachable code");
        }
    };
    ($x:expr, $msg:expr) => {
        {
            let msg = $msg.into();
            raise!($x(msg));
        }
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

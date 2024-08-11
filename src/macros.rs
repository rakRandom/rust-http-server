
#[macro_export]
macro_rules! path_to {
    ($x:expr) => { format!("static/{}", $x) };
}

pub(crate) use path_to;

#[macro_export]
macro_rules! unwrap_or_continue {
    ($res:expr, $msg:tt) => {
        match $res {
            Ok(ok) => ok,
            Err(err) => {
                println!("{}: {}", $msg, err);
                continue;
            }
        }
    };
    ($res:expr) => {
        match $res {
            Ok(ok) => ok,
            Err(err) => {
                continue;
            }
        }
    };
}

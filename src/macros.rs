#[macro_export]
macro_rules! unwrap_or_continue {
    ($res:expr, $msg:literal) => {
        match $res {
            Ok(ok) => ok,
            Err(err) => {
                println!("{}: {}", $msg, err);
                continue;
            }
        }
    };
}

#[macro_export]
macro_rules! unwrap_result_or_continue {
    ($res:expr, $msg:tt) => {
        match $res {
            Ok(ok) => ok,
            Err(err) => { println!("{}: {}", $msg, err); continue; }
        }
    };
    ($res:expr) => {
        match $res {
            Ok(ok) => ok,
            Err(err) => { continue; }
        }
    };
}

#[macro_export]
macro_rules! unwrap_option_or_continue {
    ($res:expr, $msg:tt) => {
        match $res {
            Some(some) => some,
            None => { println!("{}", $msg); continue; }
        }
    };
    ($res:expr) => {
        match $res {
            Some(some) => some,
            None => { continue; }
        }
    };
}

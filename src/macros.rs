#[macro_export]
macro_rules! unwrap_result_or_continue {
    ($res:expr, $msg:tt) => {
        match $res {
            Ok(ok) => ok,
            Err(err) => {
                if $msg.len() > 0 {
                    println!("{}: {}", $msg, err);
                } else {
                    println!("{}", err);
                }
                continue;
            }
        }
    };
    ($res:expr) => {
        match $res {
            Ok(ok) => ok,
            Err(_) => { continue; }
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

#[macro_export]
macro_rules! unwrap_result_or_break {
    ($res:expr, $msg:tt) => {
        match $res {
            Ok(ok) => ok,
            Err(err) => {
                if $msg.len() > 0 {
                    println!("{}: {}", $msg, err);
                } else {
                    println!("{}", err);
                }
                break;
            }
        }
    };
    ($res:expr) => {
        match $res {
            Ok(ok) => ok,
            Err(_) => { break; }
        }
    };
}

#[macro_export]
macro_rules! unwrap_option_or_break {
    ($res:expr, $msg:tt) => {
        match $res {
            Some(some) => some,
            None => { println!("{}", $msg); break; }
        }
    };
    ($res:expr) => {
        match $res {
            Some(some) => some,
            None => { break; }
        }
    };
}
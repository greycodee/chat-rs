

#[macro_export]
macro_rules! red_text_format {
    ($($arg:tt)*) => {
        format!("\x1b[{}m{}\x1b[0m", 31, format!($($arg)*))
    }
}

#[macro_export]
macro_rules! green_text_format {
    ($($arg:tt)*) => {
        format!("\x1b[{}m{}\x1b[0m", 32, format!($($arg)*))
    }
}

#[macro_export]
macro_rules! yellow_text_format {
    ($($arg:tt)*) => {
        format!("\x1b[{}m{}\x1b[0m", 33, format!($($arg)*))
    }
}

#[macro_export]
macro_rules! blue_text_format {
    ($($arg:tt)*) => {
        format!("\x1b[{}m{}\x1b[0m", 34, format!($($arg)*))
    }
}

#[macro_export]
macro_rules! error_format {
    ($($arg:tt)*) => {
        format!("\x1b[{}m[error] {}\x1b[0m", 31, format!($($arg)*))
    }
}

#[macro_export]
macro_rules! info_format {
    ($($arg:tt)*) => {
        format!("\x1b[{}m[info] {}\x1b[0m", 32, format!($($arg)*))
    }
}

#[macro_export]
macro_rules! warn_format {
    ($($arg:tt)*) => {
        format!("\x1b[{}m[warn] {}\x1b[0m", 33, format!($($arg)*))
    }
}

#[macro_export]
macro_rules! debug_format {
    ($($arg:tt)*) => {
        format!("\x1b[{}m[debug] {}\x1b[0m", 34, format!($($arg)*))
    }
}

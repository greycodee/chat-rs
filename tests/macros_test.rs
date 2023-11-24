use core::{
    blue_text_format, error_format, green_text_format, red_text_format, yellow_text_format,info_format,warn_format,debug_format,
};

#[test]
fn test_red_text_format() {
    let result = red_text_format!("Hello, {}", "world!");
    assert_eq!(result, "\x1b[31mHello, world!\x1b[0m");
}

#[test]
fn test_green_text_format() {
    let result = green_text_format!("Hello, {}", "world!");
    assert_eq!(result, "\x1b[32mHello, world!\x1b[0m");
}

#[test]
fn test_yellow_text_format() {
    let result = yellow_text_format!("Hello, {}", "world!");
    assert_eq!(result, "\x1b[33mHello, world!\x1b[0m");
}

#[test]
fn test_blue_text_format() {
    let result = blue_text_format!("Hello, {}", "world!");
    assert_eq!(result, "\x1b[34mHello, world!\x1b[0m");
}

#[test]
fn test_error_format() {
    let result = error_format!("Hello, {}", "world!");
    assert_eq!(result, "\x1b[31m[error] Hello, world!\x1b[0m");
}

#[test]
fn test_info_format() {
    let result = info_format!("Hello, {}", "world!");
    assert_eq!(result, "\x1b[32m[info] Hello, world!\x1b[0m");
}

#[test]
fn test_warn_format() {
    let result = warn_format!("Hello, {}", "world!");
    assert_eq!(result, "\x1b[33m[warn] Hello, world!\x1b[0m");
}

#[test]
fn test_debug_format() {
    let result = debug_format!("Hello, {}", "world!");
    assert_eq!(result, "\x1b[34m[debug] Hello, world!\x1b[0m");
}

use core::color::
{
    FONT_COLOR_GREEN,
    FONT_COLOR_RED,
    FONT_COLOR_YELLOW,
    FONT_COLOR_BLUE,
    FONT_COLOR_MAGENTA,
    FONT_COLOR_CYAN,
};
use core::{colored_format, red_format};



#[test]
fn test_colored_format_green() {
    let a = colored_format!(FONT_COLOR_GREEN, "hello");
    println!("{}",a);
    assert_eq!(a, "\x1b[32mhello\x1b[0m");
}

#[test]
fn test_colored_format_red() {
    let a = colored_format!(FONT_COLOR_RED, "hello");
    println!("{}",a);
    assert_eq!(a, "\x1b[31mhello\x1b[0m");
}

#[test]
fn test_colored_format_yellow() {
    let a = colored_format!(FONT_COLOR_YELLOW, "hello");
    println!("{}",a);
    assert_eq!(a, "\x1b[33mhello\x1b[0m");
}

#[test]
fn test_colored_format_blue() {
    let a = colored_format!(FONT_COLOR_BLUE, "hello");
    println!("{}",a);
    assert_eq!(a, "\x1b[34mhello\x1b[0m");
}

#[test]
fn test_colored_format_magenta() {
    let a = colored_format!(FONT_COLOR_MAGENTA, "hello");
    println!("{}",a);
    assert_eq!(a, "\x1b[35mhello\x1b[0m");
}

#[test]
fn test_colored_format_cyan() {
    let a = colored_format!(FONT_COLOR_CYAN, "hello");
    println!("{}",a);
    assert_eq!(a, "\x1b[36mhello\x1b[0m");
}

#[test]
fn test_colored_format_default() {
    let a = colored_format!(0, "hello");
    println!("{}",a);
    assert_eq!(a, "\x1b[0mhello\x1b[0m");
}

#[test]
fn test_red_format() {
    let a = red_format!("hello red_format");
    println!("{}",a);
    assert_eq!(a, "\x1b[31mhello red_format\x1b[0m");
}

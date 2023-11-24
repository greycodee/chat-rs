#[macro_export]
macro_rules! colored_format {
    ($color:expr, $text:expr) => {
        format!("\x1b[{}m{}\x1b[0m", $color, $text);
    };
}

#[macro_export]
macro_rules! red_format {
    ($text:expr) => {
        format!("\x1b[{}m{}\x1b[0m", FONT_COLOR_RED, $text);
    };
}
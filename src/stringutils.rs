pub trait StringUtils {
    fn with_ansi(&self, prefix: u8, suffix: u8) -> String;
    fn color(&self, color: AnsiColor) -> String;
    fn background(&self, color: AnsiColor) -> String;
    fn underline(&self) -> String;
    fn invert(&self) -> String;
    fn pad_start(&self, len: usize) -> String;
    fn pad_end(&self, len: usize) -> String;
}

// see https://misc.flogisoft.com/bash/tip_colors_and_formatting

impl StringUtils for &str {
    fn with_ansi(&self, prefix: u8, suffix: u8) -> String {
        format!("\x1B[{}m{}\x1B[{}m", prefix, self, suffix)
    }

    fn color(&self, color: AnsiColor) -> String {
        self.with_ansi(30 + color.offset(), 39)
    }

    fn background(&self, color: AnsiColor) -> String {
        self.with_ansi(40 + color.offset(), 49)
    }

    fn underline(&self) -> String {
        self.with_ansi(4, 24)
    }

    fn invert(&self) -> String {
        self.with_ansi(7, 27)
    }

    fn pad_start(&self, len: usize) -> String {
        format!("{:>0len$}", self, len = len)
    }

    fn pad_end(&self, len: usize) -> String {
        format!("{:<0len$}", self, len = len)
    }
}
impl StringUtils for String {
    fn with_ansi(&self, prefix: u8, suffix: u8) -> String {
        self.as_str().with_ansi(prefix, suffix)
    }

    fn color(&self, color: AnsiColor) -> String {
        self.as_str().color(color)
    }

    fn background(&self, color: AnsiColor) -> String {
        self.as_str().background(color)
    }

    fn underline(&self) -> String {
        self.as_str().underline()
    }
    fn invert(&self) -> String {
        self.as_str().invert()
    }
    fn pad_start(&self, len: usize) -> String {
        self.as_str().pad_start(len)
    }

    fn pad_end(&self, len: usize) -> String {
        self.as_str().pad_end(len)
    }
}

#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum AnsiColor {
    Default,
    Black,
    White,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    LightGray,
    DarkGray,
    Cyan,
    DarkRed,
    DarkGreen,
    DarkYellow,
    DarkBlue,
    DarkMagenta,
    DarkCyan,
}

pub use self::AnsiColor::*;

impl AnsiColor {
    fn offset(self) -> u8 {
        match self {
            Default => 9,
            Black => 0,
            White => 67,
            Red => 61,
            Green => 62,
            Yellow => 63,
            Blue => 64,
            Magenta => 65,
            LightGray => 7,
            DarkGray => 60,
            Cyan => 66,
            DarkRed => 1,
            DarkGreen => 2,
            DarkYellow => 3,
            DarkBlue => 4,
            DarkMagenta => 5,
            DarkCyan => 6,
        }
    }
}

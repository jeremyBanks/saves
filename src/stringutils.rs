pub trait StringUtils {
    fn color(&self, color: Color) -> String;
    fn background(&self, color: Color) -> String;
    fn underline(&self) -> String;
    fn invert(&self) -> String;
    fn pad_start(&self, len: usize) -> String;
    fn pad_end(&self, len: usize) -> String;
}

impl StringUtils for &str {
    fn color(&self, color: Color) -> String {
        format!(
            "<span style=\"color: {};\">{self}</span>",
            color.css_color(),
        )
    }

    fn background(&self, color: Color) -> String {
        format!(
            "<span style=\"background-color: {};\">{self}</span>",
            color.css_color(),
        )
    }

    fn underline(&self) -> String {
        format!("<span style=\"text-decoration: underline;\">{self}</span>")
    }

    fn invert(&self) -> String {
        format!("<span style=\"font-weight: bold;\">{}</span>", self)
    }

    fn pad_start(&self, len: usize) -> String {
        format!("{self:>0len$}")
    }

    fn pad_end(&self, len: usize) -> String {
        format!("{self:<0len$}")
    }
}
impl StringUtils for String {
    fn color(&self, color: Color) -> String {
        self.as_str().color(color)
    }

    fn background(&self, color: Color) -> String {
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
pub enum Color {
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

pub use self::Color::*;

impl Color {
    fn css_color(self) -> &'static str {
        match self {
            Default => "default",
            Black => "#000",
            White => "#FFF",
            Red => "#EF2929",
            Green => "#8AE234",
            Yellow => "#FCE94F",
            Blue => "#32AFFF",
            Magenta => "#AD7FA8",
            LightGray => "#D3D7CF",
            DarkGray => "#555753",
            Cyan => "#34E2E2",
            DarkRed => "#C00",
            DarkGreen => "#4E9A06",
            DarkYellow => "#C4A000",
            DarkBlue => "#729FCF",
            DarkMagenta => "#75507B",
            DarkCyan => "#06989A",
        }
    }
}

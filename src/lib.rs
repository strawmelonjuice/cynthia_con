use std::env;

pub fn horizline() -> String {
    "\u{2500}"
        .repeat(termsize::get().unwrap().cols as usize)
        .to_string()
}
type CynthiaStyledString = String;

pub trait CynthiaStyles {
    /// Display the string with a bold effect.
    fn style_bold(self) -> CynthiaStyledString;
    /// Display the string with an italic effect.
    fn style_italic(self) -> CynthiaStyledString;
    /// Display the string with an underline effect.
    fn style_underline(self) -> CynthiaStyledString;
    /// Display the string with a strikethrough effect.
    fn style_strikethrough(self) -> CynthiaStyledString;
    /// Display the string with a dim effect.
    fn style_dim(self) -> CynthiaStyledString;
    /// Display the string with a blinking effect.
    fn style_blink(self) -> CynthiaStyledString;
    /// Invert the colors of the string.
    fn style_reverse(self) -> CynthiaStyledString;
    /// Attempts to center the string in the terminal. This requires a new line.
    fn style_centered(self) -> CynthiaStyledString;
    /// Clears all styles.
    fn style_clear(self) -> CynthiaStyledString;
}
impl CynthiaStyles for &str {
    #[inline]
    fn style_bold(self) -> CynthiaStyledString {
        format!("\u{001b}[1m{}\u{001b}[0m", self)
    }
    #[inline]
    fn style_italic(self) -> CynthiaStyledString {
        format!("\u{001b}[3m{}\u{001b}[0m", self)
    }
    #[inline]
    fn style_underline(self) -> CynthiaStyledString {
        format!("\u{001b}[4m{}\u{001b}[0m", self)
    }
    #[inline]
    fn style_strikethrough(self) -> CynthiaStyledString {
        format!("\u{001b}[9m{}\u{001b}[0m", self)
    }
    #[inline]
    fn style_dim(self) -> CynthiaStyledString {
        format!("\u{001b}[2m{}\u{001b}[0m", self)
    }
    #[inline]
    fn style_blink(self) -> CynthiaStyledString {
        format!("\u{001b}[5m{}\u{001b}[0m", self)
    }
    #[inline]
    fn style_reverse(self) -> CynthiaStyledString {
        format!("\u{001b}[7m{}\u{001b}[0m", self)
    }
    #[inline]
    fn style_clear(self) -> CynthiaStyledString {
        format!("\u{001b}[0m{}\u{001b}[0m", self)
    }
    #[inline]
    fn style_centered(self) -> CynthiaStyledString {
        let cols = termsize::get().unwrap().cols as usize;
        let len = self.chars().filter(|c| c.is_alphanumeric()).count();
        let spaces = (cols - len) / 3;
        format!("{0}{1}{0}", " ".repeat(spaces), self)
    }
}
impl CynthiaStyles for String {
    #[inline]
    fn style_bold(self) -> CynthiaStyledString {
        format!("\u{001b}[1m{}\u{001b}[0m", self)
    }
    #[inline]
    fn style_italic(self) -> CynthiaStyledString {
        format!("\u{001b}[3m{}\u{001b}[0m", self)
    }
    #[inline]
    fn style_underline(self) -> CynthiaStyledString {
        format!("\u{001b}[4m{}\u{001b}[0m", self)
    }
    #[inline]
    fn style_strikethrough(self) -> CynthiaStyledString {
        format!("\u{001b}[9m{}\u{001b}[0m", self)
    }
    #[inline]
    fn style_dim(self) -> CynthiaStyledString {
        format!("\u{001b}[2m{}\u{001b}[0m", self)
    }
    #[inline]
    fn style_blink(self) -> CynthiaStyledString {
        format!("\u{001b}[5m{}\u{001b}[0m", self)
    }
    #[inline]
    fn style_reverse(self) -> CynthiaStyledString {
        format!("\u{001b}[7m{}\u{001b}[0m", self)
    }
    #[inline]
    fn style_clear(self) -> CynthiaStyledString {
        format!("\u{001b}[0m{}\u{001b}[0m", self)
    }
    #[inline]
    fn style_centered(self) -> CynthiaStyledString {
        let cols = termsize::get().unwrap().cols as usize;
        let len = self.chars().filter(|c| c.is_alphanumeric()).count();
        let spaces = (cols - len) / 2;
        format!("{0}{1}{0}", " ".repeat(spaces), self)
    }
}
type CynthiaColoredString = String;

pub trait CynthiaColors {
    fn by_rgb(self, r: u32, g: u32, b: u32) -> CynthiaColoredString;
    fn color_green(self) -> CynthiaColoredString;
    fn color_ok_green(self) -> CynthiaColoredString;
    fn color_lime(self) -> CynthiaColoredString;
    fn color_red(self) -> CynthiaColoredString;
    fn color_error_red(self) -> CynthiaColoredString;
    fn color_bright_red(self) -> CynthiaColoredString;
    fn color_black(self) -> CynthiaColoredString;
    fn color_bright_black(self) -> CynthiaColoredString;
    fn color_white(self) -> CynthiaColoredString;
    fn color_bright_white(self) -> CynthiaColoredString;
    fn color_yellow(self) -> CynthiaColoredString;
    fn color_bright_yellow(self) -> CynthiaColoredString;
    fn color_cyan(self) -> CynthiaColoredString;
    fn color_bright_cyan(self) -> CynthiaColoredString;
    fn color_magenta(self) -> CynthiaColoredString;
    fn color_pink(self) -> CynthiaColoredString;
    fn color_blue(self) -> CynthiaColoredString;
    fn color_lightblue(self) -> CynthiaColoredString;
    fn color_orange(self) -> CynthiaColoredString;
    fn color_bright_orange(self) -> CynthiaColoredString;
    fn color_purple(self) -> CynthiaColoredString;
    fn color_lilac(self) -> CynthiaColoredString;
}
impl CynthiaColors for &str {
    #[inline]
    fn by_rgb(self, r: u32, g: u32, b: u32) -> CynthiaColoredString {
        match env::var("NO_COLOR") {
            Ok(a) if String::is_empty(&a) => format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, self),
            Ok(_) => self.to_string(),
            Err(_) => format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, self),
        }
    }
    #[inline]
    fn color_green(self) -> CynthiaColoredString {
        self.by_rgb(0, 255, 0)
    }
    #[inline]
    fn color_ok_green(self) -> CynthiaColoredString {
        self.by_rgb(116, 204, 140)
    }
    #[inline]
    fn color_lime(self) -> CynthiaColoredString {
        self.by_rgb(66, 245, 78)
    }
    #[inline]
    fn color_red(self) -> CynthiaColoredString {
        self.by_rgb(255, 0, 0)
    }
    #[inline]
    fn color_error_red(self) -> CynthiaColoredString {
        self.by_rgb(184, 28, 74)
    }
    #[inline]
    fn color_bright_red(self) -> CynthiaColoredString {
        self.by_rgb(237, 68, 62)
    }
    #[inline]
    fn color_black(self) -> CynthiaColoredString {
        self.by_rgb(41, 40, 40)
    }
    #[inline]
    fn color_bright_black(self) -> CynthiaColoredString {
        self.by_rgb(0, 0, 0)
    }
    #[inline]
    fn color_white(self) -> CynthiaColoredString {
        self.by_rgb(240, 240, 240)
    }
    #[inline]
    fn color_bright_white(self) -> CynthiaColoredString {
        self.by_rgb(255, 255, 255)
    }
    #[inline]
    fn color_yellow(self) -> CynthiaColoredString {
        self.by_rgb(243, 201, 35)
    }
    #[inline]
    fn color_bright_yellow(self) -> CynthiaColoredString {
        self.by_rgb(255, 234, 150)
    }
    #[inline]
    fn color_cyan(self) -> CynthiaColoredString {
        self.by_rgb(16, 227, 227)
    }
    #[inline]
    fn color_bright_cyan(self) -> CynthiaColoredString {
        self.by_rgb(0, 255, 255)
    }
    #[inline]
    fn color_magenta(self) -> CynthiaColoredString {
        self.by_rgb(255, 0, 255)
    }
    #[inline]
    fn color_pink(self) -> CynthiaColoredString {
        self.by_rgb(243, 154, 245)
    }
    #[inline]
    fn color_blue(self) -> CynthiaColoredString {
        self.by_rgb(0, 0, 255)
    }
    #[inline]
    fn color_lightblue(self) -> CynthiaColoredString {
        self.by_rgb(145, 220, 255)
    }
    #[inline]
    fn color_orange(self) -> CynthiaColoredString {
        self.by_rgb(255, 165, 0)
    }
    #[inline]
    fn color_bright_orange(self) -> CynthiaColoredString {
        self.by_rgb(255, 157, 0)
    }
    #[inline]
    fn color_purple(self) -> CynthiaColoredString {
        self.by_rgb(97, 18, 181)
    }
    #[inline]
    fn color_lilac(self) -> CynthiaColoredString {
        self.by_rgb(200, 162, 200)
    }
}
impl CynthiaColors for String {
    #[inline]
    fn by_rgb(self, r: u32, g: u32, b: u32) -> CynthiaColoredString {
        self.as_str().by_rgb(r, g, b)
    }
    #[inline]
    fn color_green(self) -> CynthiaColoredString {
        self.as_str().color_green()
    }
    #[inline]
    fn color_ok_green(self) -> CynthiaColoredString {
        self.as_str().color_ok_green()
    }
    #[inline]
    fn color_lime(self) -> CynthiaColoredString {
        self.as_str().color_lime()
    }
    #[inline]
    fn color_red(self) -> CynthiaColoredString {
        self.as_str().color_red()
    }
    #[inline]
    fn color_error_red(self) -> CynthiaColoredString {
        self.as_str().color_error_red()
    }
    #[inline]
    fn color_bright_red(self) -> CynthiaColoredString {
        self.as_str().color_bright_red()
    }
    #[inline]
    fn color_black(self) -> CynthiaColoredString {
        self.as_str().color_black()
    }
    #[inline]
    fn color_bright_black(self) -> CynthiaColoredString {
        self.as_str().color_bright_black()
    }
    #[inline]
    fn color_white(self) -> CynthiaColoredString {
        self.as_str().color_white()
    }
    #[inline]
    fn color_bright_white(self) -> CynthiaColoredString {
        self.as_str().color_bright_white()
    }
    #[inline]
    fn color_yellow(self) -> CynthiaColoredString {
        self.as_str().color_yellow()
    }
    #[inline]
    fn color_bright_yellow(self) -> CynthiaColoredString {
        self.as_str().color_bright_yellow()
    }
    #[inline]
    fn color_cyan(self) -> CynthiaColoredString {
        self.as_str().color_cyan()
    }
    #[inline]
    fn color_bright_cyan(self) -> CynthiaColoredString {
        self.as_str().color_bright_cyan()
    }
    #[inline]
    fn color_magenta(self) -> CynthiaColoredString {
        self.as_str().color_magenta()
    }
    #[inline]
    fn color_pink(self) -> CynthiaColoredString {
        self.as_str().color_pink()
    }
    #[inline]
    fn color_blue(self) -> CynthiaColoredString {
        self.as_str().color_blue()
    }
    #[inline]
    fn color_lightblue(self) -> CynthiaColoredString {
        self.as_str().color_lightblue()
    }
    #[inline]
    fn color_orange(self) -> CynthiaColoredString {
        self.as_str().color_orange()
    }
    #[inline]
    fn color_bright_orange(self) -> CynthiaColoredString {
        self.as_str().color_bright_orange()
    }
    #[inline]
    fn color_purple(self) -> CynthiaColoredString {
        self.as_str().color_purple()
    }
    #[inline]
    fn color_lilac(self) -> CynthiaColoredString {
        self.as_str().color_lilac()
    }
}

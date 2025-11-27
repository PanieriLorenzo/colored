use std::{convert::Infallible, str::FromStr};

use crate::{style, Color, Style, Styles};

/// Borrowed version of [`crate::ColoredString`]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct ColoredStr<'a> {
    /// The plain text that will have color and style applied to it.
    pub input: &'a str,
    /// The color of the text as it will be printed.
    pub fgcolor: Option<Color>,
    /// The background color (if any). None means that the text will be printed
    /// without a special background.
    pub bgcolor: Option<Color>,
    /// Any special styling to be applied to the text (see Styles for a list of
    /// available options).
    pub style: style::Style,
}

impl<'a> ColoredStr<'a> {
    /// Creates a new [`ColoredStr`] from a string slice with no formatting applied.
    ///
    /// Note that this method does not allocate.
    pub const fn new(str: &'a str) -> Self {
        Self {
            input: str,
            fgcolor: None,
            bgcolor: None,
            style: Style::new(),
        }
    }

    /// Clears foreground coloring on this [`ColoredStr`], meaning that it
    /// will be printed with the default terminal text color.
    pub const fn clear_fgcolor(&mut self) {
        self.fgcolor = None;
    }

    /// Gets rid of this [`ColoredStr`]'s background.
    pub const fn clear_bgcolor(&mut self) {
        self.bgcolor = None;
    }

    /// Clears any special styling and sets it back to the default (plain,
    /// maybe colored, text).
    pub const fn clear_style(&mut self) {
        self.style = Style::new();
    }

    /// Checks if the colored string has no color or styling.
    #[must_use]
    pub const fn is_plain(&self) -> bool {
        self.bgcolor.is_none() && self.fgcolor.is_none() && self.style.const_eq(&style::CLEAR)
    }

    pub const fn const_color(self, color: Color) -> ColoredStr<'a> {
        Self {
            fgcolor: Some(color),
            ..self
        }
    }

    pub const fn const_on_color(self, color: Color) -> ColoredStr<'a> {
        Self {
            bgcolor: Some(color),
            ..self
        }
    }

    pub const fn const_clear(self) -> ColoredStr<'a> {
        Self::new(self.input)
    }

    pub const fn const_normal(self) -> ColoredStr<'a> {
        self.const_clear()
    }

    pub const fn const_bold(mut self) -> ColoredStr<'a> {
        self.style.add(Styles::Bold);
        self
    }

    pub const fn const_dimmed(mut self) -> ColoredStr<'a> {
        self.style.add(style::Styles::Dimmed);
        self
    }

    pub const fn const_italic(mut self) -> ColoredStr<'a> {
        self.style.add(style::Styles::Italic);
        self
    }

    pub const fn const_underline(mut self) -> ColoredStr<'a> {
        self.style.add(style::Styles::Underline);
        self
    }

    pub const fn const_blink(mut self) -> ColoredStr<'a> {
        self.style.add(style::Styles::Blink);
        self
    }

    pub const fn const_reverse(self) -> ColoredStr<'a> {
        self.const_reversed()
    }

    pub const fn const_reversed(mut self) -> ColoredStr<'a> {
        self.style.add(style::Styles::Reversed);
        self
    }

    pub const fn const_hidden(mut self) -> ColoredStr<'a> {
        self.style.add(style::Styles::Hidden);
        self
    }

    pub const fn const_strikethrough(mut self) -> ColoredStr<'a> {
        self.style.add(style::Styles::Strikethrough);
        self
    }
}

#[cfg(test)]
mod tests {

    use crate::{colored_str::ColoredStr, Color, Style};

    #[test]
    fn const_string_creation() {
        const _STRING: ColoredStr = ColoredStr::new("foo")
            .const_on_color(Color::Blue)
            .const_color(Color::Red)
            .const_bold();
    }
}

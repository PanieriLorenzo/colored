use crate::{style, Color};

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

#[cfg(test)]
mod tests {

    use crate::{colored_str::ColoredStr, Color, Style};

    #[test]
    fn const_string_creation() {
        const _STRING: ColoredStr = ColoredStr {
            input: "foo",
            fgcolor: Some(Color::Magenta),
            bgcolor: Some(Color::Green),
            style: Style::new(),
        };
    }
}

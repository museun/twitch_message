/// A RGB color triplet
///
/// # Examples
/// ### Parsing:
/// ```rust
/// # use twitch_message::Color;
/// # use std::str::FromStr;
/// let color: Color = "#ff00ff".parse().unwrap();
/// assert_eq!(color, Color(0xFF, 0x00, 0xFF));
///
/// let color: Color = "abcdef".parse().unwrap();
/// assert_eq!(color, Color(0xAB, 0xCD, 0xEF));
///
/// let color: Color = Color::from_str("123456").unwrap();
/// assert_eq!(color, Color(0x12, 0x34, 0x56));
/// ```
/// ### Formatting:
/// ```rust
/// # use twitch_message::Color;
/// let color = Color(0xDF, 0x12, 0x34);
/// assert_eq!(color.to_string(), "#DF1234")
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Color(
    /// Red
    pub u8,
    /// Green
    pub u8,
    /// Blue
    pub u8,
);

impl Color {
    /// Get the red channel
    pub const fn red(&self) -> u8 {
        self.0
    }

    /// Get the green channel
    pub const fn green(&self) -> u8 {
        self.1
    }

    /// Get the blue channel
    pub const fn blue(&self) -> u8 {
        self.2
    }
}

impl Default for Color {
    /// The default for a Color is white (`0xFFFFFF`)
    fn default() -> Self {
        Self(0xFF, 0xFF, 0xFF)
    }
}

impl std::fmt::Display for Color {
    /// Constructs a `#RRGGBB` string from this type
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(r, g, b) = self;
        write!(f, "#{r:02X}{g:02X}{b:02X}")
    }
}

impl std::str::FromStr for Color {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = match input.len() {
            7 => &input[1..],
            6 => input,
            _ => return Err("invalid hex format"),
        };

        let color = u32::from_str_radix(input, 16).map_err(|_| "invalid hex digit")?;
        let (r, g, b) = (
            ((color >> 16) & 0xFF) as _,
            ((color >> 8) & 0xFF) as _,
            (color & 0xFF) as _,
        );
        Ok(Self(r, g, b))
    }
}

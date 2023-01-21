#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub const fn red(&self) -> u8 {
        self.0
    }

    pub const fn green(&self) -> u8 {
        self.1
    }

    pub const fn blue(&self) -> u8 {
        self.2
    }
}

impl Default for Color {
    fn default() -> Self {
        Self(0xFF, 0xFF, 0xFF)
    }
}

impl std::fmt::Display for Color {
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

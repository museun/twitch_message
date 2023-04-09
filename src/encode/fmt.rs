use core::fmt::Write;

/// A trait to format a type onto this [`core::fmt::Write`]
pub trait Format: Write {
    /// Format the message
    fn format_msg(&mut self, msg: impl Formattable) -> core::fmt::Result {
        msg.format(self)
    }
}

impl<T> Format for T where T: Write {}

/// A trait for formatting a message with [`core::fmt::Write`]
pub trait Formattable: std::fmt::Display {
    /// Format the message
    fn format(&self, fmt: impl Write) -> core::fmt::Result;
}

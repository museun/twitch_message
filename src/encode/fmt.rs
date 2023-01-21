use core::fmt::Write;

pub trait Format: Write {
    fn format_msg(&mut self, msg: impl Formattable) -> core::fmt::Result {
        msg.format(self)
    }
}

impl<T> Format for T where T: Write {}

pub trait Formattable {
    fn format(&self, fmt: impl Write) -> core::fmt::Result;
}

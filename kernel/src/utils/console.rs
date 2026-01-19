use core::fmt::{self, Write, Arguments};
use crate::utils::sbi::console_putchar;

struct Stdout;

impl Write for Stdout {
    /// write str to console
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

/// print to the host console using the format string and arguments.
pub fn _print(args: Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// Print! macro to the host console using the format string and arguments.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::console::_print(format_args!("{}", format_args!($($arg)*)))
    };
}

/// Println! macro to the host console using the format string and arguments.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => {
        $crate::print!($($arg)*);
        $crate::print!("\n")
    };
}
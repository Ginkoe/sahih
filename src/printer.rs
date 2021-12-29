use std::io::{self, Write};

use crate::{config::ConfigManager, constants::VERSION};

pub trait Print {
    fn print(&mut self, value: &str) -> io::Result<()>;
    fn println(&mut self, value: &str) -> io::Result<()>;
}

pub trait PrintHelper {
    fn print_welcome(&mut self) -> io::Result<()>;
    fn print_target_projects(&mut self, config: &ConfigManager) -> io::Result<()>;

    fn print_styled(&mut self, value: &str, opts: PrintOptions) -> io::Result<()>;
    fn println_styled(&mut self, value: &str, opts: PrintOptions) -> io::Result<()>;
}

pub struct Printer<W> {
    writer: W,
}

#[derive(Clone, Copy)]
pub struct PrintOptions {
    color: termcolor::Color,
    is_bold: bool,
}

impl<W: Write + termcolor::WriteColor> Printer<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W: Write> Print for Printer<W> {
    fn print(&mut self, value: &str) -> io::Result<()> {
        write!(self.writer, "{}", value)
    }

    fn println(&mut self, value: &str) -> io::Result<()> {
        writeln!(self.writer, "{}", value)
    }
}

impl<W: Write + termcolor::WriteColor> PrintHelper for Printer<W> {
    fn print_welcome(&mut self) -> io::Result<()> {
        let welcome_message = format!("🔎 Starting execution of Sahih v{}", VERSION);

        ColorPalette::BoldGreen.println(&welcome_message, self)
    }

    fn print_target_projects(&mut self, config: &ConfigManager) -> io::Result<()> {
        self.println(&format!(
            "Found {} target projects : ",
            config.projects.len()
        ))?;

        for project in config.projects.keys() {
            ColorPalette::Blue.println(&format!("- {}", project), self)?;
        }

        Ok(())
    }

    fn print_styled(&mut self, value: &str, opts: PrintOptions) -> io::Result<()> {
        let mut color_spec = termcolor::ColorSpec::new();
        color_spec.set_fg(Some(opts.color)).set_bold(opts.is_bold);
        self.writer.set_color(&color_spec)?;
        write!(self.writer, "{}", value)?;
        self.writer.reset()
    }

    fn println_styled(&mut self, value: &str, opts: PrintOptions) -> io::Result<()> {
        let mut color_spec = termcolor::ColorSpec::new();
        color_spec.set_fg(Some(opts.color)).set_bold(opts.is_bold);
        self.writer.set_color(&color_spec)?;
        writeln!(self.writer, "{}", value)?;
        self.writer.reset()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use crate::printer::{PrintHelper, PrintOptions, Printer};

    #[test]
    fn test_printer__println_styled__success() {
        let mut output_1 = termcolor::Ansi::new(vec![]);
        let mut printer = Printer::new(&mut output_1);

        let opts_green_bold = PrintOptions {
            color: termcolor::Color::Green,
            is_bold: true,
        };

        printer
            .println_styled("some-green-bold-text", opts_green_bold)
            .unwrap();

        let actual_green_bold = String::from_utf8(output_1.into_inner()).unwrap();
        let expected_green_bold = "\u{1b}[0m\u{1b}[1m\u{1b}[32msome-green-bold-text\n\u{1b}[0m";

        assert_eq!(actual_green_bold, expected_green_bold);

        let mut output_2 = termcolor::Ansi::new(vec![]);
        printer = Printer::new(&mut output_2);

        let opts_yellow = PrintOptions {
            color: termcolor::Color::Yellow,
            is_bold: false,
        };

        printer
            .println_styled("some-yellow-text", opts_yellow)
            .unwrap();

        let actual_yellow = String::from_utf8(output_2.into_inner()).unwrap();
        let expected_yellow = "\u{1b}[0m\u{1b}[33msome-yellow-text\n\u{1b}[0m";

        assert_eq!(actual_yellow, expected_yellow);
    }
}

pub enum ColorPalette {
    BoldGreen,
    Blue,
}

impl ColorPalette {
    pub fn to_color(&self) -> PrintOptions {
        match self {
            ColorPalette::BoldGreen => PrintOptions {
                color: termcolor::Color::Green,
                is_bold: true,
            },
            ColorPalette::Blue => PrintOptions {
                color: termcolor::Color::Blue,
                is_bold: false,
            },
        }
    }

    pub fn print<W: Print + PrintHelper>(&mut self, value: &str, writer: &mut W) -> io::Result<()> {
        writer.print_styled(value, self.to_color())?;
        Ok(())
    }

    pub fn println<W: Print + PrintHelper>(
        &mut self,
        value: &str,
        writer: &mut W,
    ) -> io::Result<()> {
        writer.println_styled(value, self.to_color())?;
        Ok(())
    }
}

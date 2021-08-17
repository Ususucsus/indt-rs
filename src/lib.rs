//! Provides writer for writing with indentions.
//!
//! This crate contains struct `Indent` which provides functionallity to write with indentions.
//! It is also allows to configure indention style and depth.
//!
//! There is several crates with greater functionallity, most popular is [indenter].
//!
//! [indenter]: https://crates.io/crates/indenter/

use std::cmp;
use std::io;

/// Represent struct used for printing with indentions.
///
/// Create it with indention style options with `new` method or
/// with default styles via `from_writer` method.
/// Use `more` and `less` methods to specify indention depth.
///
/// # Examples
///
/// ```
/// use indt::Indent;
/// use std::io::Write;
/// use std::io;
///
/// let stdout = &mut io::stdout();
/// let mut indent = Indent::from_writer(stdout);
///
/// indent.more();
///
/// writeln!(indent, "lorem ipsum"); // "    lorem ipsum" printed
///
/// ```
pub struct Indent<'a> {
    output: &'a mut dyn io::Write,
    indent_step: u8,
    indent_symbol: char,
    current_indent: u8,
    first_line: bool,
}

impl<'a> Indent<'a> {
    /// Creates a new instance of the `Indent` struct with default indent options.
    /// Default indent character is whitespace `' '` and default indent is 4 characters long.
    ///
    /// ## Arguments
    ///
    /// * `output` - Writing destination
    ///
    /// # Examples
    ///
    /// ```
    /// use indt::Indent;
    /// use std::io::Write;
    ///
    /// let mut buffer = Vec::new();
    /// let mut indent = Indent::from_writer(&mut buffer);
    ///
    /// indent.more();
    ///
    /// write!(indent, "lorem ipsum");
    ///
    /// assert_eq!("    lorem ipsum", String::from_utf8_lossy(&buffer));
    ///
    /// ```
    pub fn from_writer(output: &'a mut dyn io::Write) -> Indent<'a> {
        Self::new(output, 4, ' ')
    }

    /// Creates a new instance of the `Indent` struct with specified indent options.
    ///
    /// ## Arguments
    ///
    /// * `output` - Writing destination
    /// * `indent_step` - Size of one indent in characters
    /// * `indent_symbol` - Character that will be used to write indent
    ///
    /// # Examples
    ///
    /// ```
    /// use indt::Indent;
    /// use std::io::Write;
    ///
    /// let mut buffer = Vec::new();
    /// let mut indent = Indent::new(&mut buffer, 3, '-');
    ///
    /// indent.more();
    ///
    /// write!(indent, "lorem ipsum");
    ///
    /// assert_eq!("---lorem ipsum", String::from_utf8_lossy(&buffer));
    ///
    /// ```
    pub fn new(output: &'a mut dyn io::Write, indent_step: u8, indent_symbol: char) -> Indent<'a> {
        Indent {
            output,
            indent_step,
            indent_symbol,
            current_indent: 0,
            first_line: true,
        }
    }

    /// Increases indent by `indent_step` specified in `new` method.
    /// Maximum value is `u8::MAX`.
    /// Initial indent set to `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use indt::Indent;
    /// use std::io::Write;
    ///
    /// let mut buffer = Vec::new();
    /// let mut indent = Indent::new(&mut buffer, 2, '.');
    ///
    /// indent.more().more().more();
    ///
    /// write!(indent, "lorem ipsum");
    ///
    /// assert_eq!("......lorem ipsum", String::from_utf8_lossy(&buffer));
    ///
    /// ```
    pub fn more(&mut self) -> &mut Indent<'a> {
        let next_indent = self.current_indent as u16 + self.indent_step as u16;
        self.current_indent = cmp::min(next_indent, u8::MAX as u16) as u8;

        self
    }

    /// Decreases indent by `indent_step` specified in `new` method.
    /// Minimum value is `u8::MIN`
    /// Initial indent set to `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use indt::Indent;
    /// use std::io::Write;
    ///
    /// let mut buffer = Vec::new();
    /// let mut indent = Indent::new(&mut buffer, 2, '.');
    ///
    /// indent.more().more().less();
    ///
    /// write!(indent, "lorem ipsum");
    ///
    /// assert_eq!("..lorem ipsum", String::from_utf8_lossy(&buffer));
    ///
    /// ```
    pub fn less(&mut self) -> &mut Indent<'a> {
        let next_indent = self.current_indent as i16 - self.indent_step as i16;
        self.current_indent = cmp::max(next_indent, u8::MIN as i16) as u8;

        self
    }

    /// Writes indent with `indent_symbol` and `current_indent` length long to `output`.
    fn write_indent(&mut self) -> Result<(), io::Error> {
        for _ in 0..self.current_indent {
            write!(self.output, "{}", self.indent_symbol)?;
        }

        Ok(())
    }
}

impl<'a> io::Write for Indent<'a> {
    fn write(&mut self, s: &[u8]) -> Result<usize, io::Error> {
        if self.first_line {
            self.write_indent()?;
            self.first_line = false;
        }

        let mut splitted = s.split(|x| *x == b'\n');
        let mut printed: usize = 0;

        if let Some(first) = splitted.next() {
            printed += self.output.write(first)?;

            for line in splitted {
                printed += self.output.write(b"\n")?;

                if !line.is_empty() {
                    self.write_indent()?;
                    printed += self.output.write(line)?;
                } else {
                    self.first_line = true;
                }
            }
        }

        Ok(printed)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.output.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    pub fn first_line_without_indent() {
        let mut buffer = Vec::new();
        let mut indt = Indent::from_writer(&mut buffer);

        write!(indt, "first line").unwrap();

        assert_eq!("first line", String::from_utf8_lossy(&buffer));
    }

    #[test]
    pub fn first_line_with_one_indent() {
        let mut buffer = Vec::new();
        let mut indt = Indent::from_writer(&mut buffer);

        indt.more();
        write!(indt, "first line").unwrap();

        assert_eq!("    first line", String::from_utf8_lossy(&buffer));
    }

    #[test]
    pub fn first_line_with_multiple_indent() {
        let mut buffer = Vec::new();
        let mut indt = Indent::from_writer(&mut buffer);

        indt.more().more().more();
        write!(indt, "first line").unwrap();

        assert_eq!("            first line", String::from_utf8_lossy(&buffer));
    }

    #[test]
    pub fn multiple_lines() {
        let mut buffer = Vec::new();
        let mut indt = Indent::from_writer(&mut buffer);

        writeln!(indt, "{} first line", 1).unwrap();

        indt.more();

        writeln!(indt, "second {} line", 2).unwrap();

        indt.more().more();

        writeln!(indt, "third line {}", 3).unwrap();

        indt.less();

        writeln!(indt, "fourth line").unwrap();

        write!(indt, "fifth line").unwrap();
        #[allow(clippy::write_with_newline)]
        write!(indt, "also fifth line\n").unwrap();

        write!(indt, "sixth line").unwrap();

        assert_eq!(
            "1 first line\n    second 2 line\n            third line 3\n        fourth line\n        fifth linealso fifth line\n        sixth line",
            String::from_utf8_lossy(&buffer)
        )
    }

    #[test]
    pub fn less() {
        let mut buffer = Vec::new();
        let mut indt = Indent::from_writer(&mut buffer);

        indt.less();

        writeln!(indt, "first line").unwrap();

        assert_eq!("first line\n", String::from_utf8_lossy(&buffer));
    }

    #[test]
    pub fn more() {
        let mut buffer = Vec::new();
        let mut indt = Indent::from_writer(&mut buffer);

        for _ in 0..300 {
            indt.more();
        }

        writeln!(indt, "first line").unwrap();

        assert_eq!(
            "                                                                                                                                                                                                                                                               first line\n", 
            String::from_utf8_lossy(&buffer)
        );
    }

    #[test]
    pub fn custom_indent() {
        let mut buffer = Vec::new();
        let mut indt = Indent::new(&mut buffer, 2, '.');

        indt.more();

        write!(indt, "first line").unwrap();

        indt.more();

        write!(indt, "\nsecond line").unwrap();

        assert_eq!(
            "..first line\n....second line",
            String::from_utf8_lossy(&buffer)
        );
    }

    #[test]
    pub fn empty_lines() {
        let mut buffer = Vec::new();
        let mut indt = Indent::from_writer(&mut buffer);

        indt.more();

        write!(indt, "first line\n\nsecond line").unwrap();

        assert_eq!(
            "    first line\n\n    second line",
            String::from_utf8_lossy(&buffer)
        );
    }
}

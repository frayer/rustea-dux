use std::io::{stdout, Result, Stdout};

use crossterm::{
    cursor, execute,
    style::Print,
    terminal::{Clear, ClearType},
};

/// Prints multiline text to the terminal.
///
/// Multiline text must be separated by the newline character. The previously printed lines are
/// cleared before printing the new text.
pub struct StringPrinter {
    prev: String,
    curr: String,
    stdout: Stdout,
}

impl StringPrinter {
    pub fn new() -> Self {
        Self {
            prev: String::new(),
            curr: String::new(),
            stdout: stdout(),
        }
    }

    /// Update the text to be printed. Make sure to only call this once before printing. Multiple
    /// invocations will cause the previously printed text to be forgotten which in turn breaks the
    /// ability to clear previously printed lines.
    pub fn update(&mut self, view: String) {
        self.prev = self.curr.clone();
        self.curr = normalized_view(view);
    }

    /// Clear previously printed lines and print the new text to the terminal.
    pub fn print(&self) -> Result<()> {
        self.clear_lines()?;
        execute!(&self.stdout, Print(&self.curr))?;

        Ok(())
    }

    fn clear_lines(&self) -> Result<()> {
        let count = self.prev.matches("\r\n").count();
        for _ in 0..count {
            execute!(
                &self.stdout,
                cursor::MoveToPreviousLine(1),
                Clear(ClearType::CurrentLine)
            )?;
        }

        Ok(())
    }
}

fn normalized_view(view: String) -> String {
    let view = if !view.ends_with('\n') {
        view + "\n"
    } else {
        view
    };
    view.replace('\n', "\r\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalized_view_without_trailing_newline() {
        let view = String::from("test");
        let expected = String::from("test\r\n");
        assert_eq!(normalized_view(view), expected);
    }

    #[test]
    fn test_normalized_view_with_trailing_newline() {
        let view = String::from("test\n");
        let expected = String::from("test\r\n");
        assert_eq!(normalized_view(view), expected);
    }

    #[test]
    fn test_normalized_view_with_multiple_newlines() {
        let view = String::from("test\n\ncase");
        let expected = String::from("test\r\n\r\ncase\r\n");
        assert_eq!(normalized_view(view), expected);
    }
}

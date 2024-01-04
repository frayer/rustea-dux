use std::io;

use crossterm::{
    cursor,
    event::{KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

use rusteadux::{command, App, Command, Message, ResizeEvent};

struct Model {
    terminal_x: u16,
    terminal_y: u16,
    moved: bool,
}

impl App for Model {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Some(resize_event) = msg.downcast_ref::<ResizeEvent>() {
            self.moved = true;
            self.terminal_x = resize_event.0;
            self.terminal_y = resize_event.1;
        } else if let Ok(key_event) = msg.downcast::<KeyEvent>() {
            if let KeyModifiers::CONTROL = key_event.modifiers {
                match key_event.code {
                    KeyCode::Char('c') => return Some(Box::new(command::quit)),
                    _ => return None,
                }
            }
        }

        None
    }

    fn view(&self) {
        let msg = if self.moved {
            format!(
                "Terminal size: (x: {}, y: {})",
                self.terminal_x, self.terminal_y
            )
        } else {
            "Resize the terminal to see effect".to_string()
        };

        execute!(
            io::stdout(),
            cursor::MoveTo(0, 0),
            Print(msg),
            Clear(ClearType::UntilNewLine),
        )
        .unwrap();
    }
}

fn main() {
    let model = Model {
        terminal_x: 0,
        terminal_y: 0,
        moved: false,
    };

    execute!(
        io::stdout(),
        cursor::MoveTo(0, 0),
        Clear(ClearType::FromCursorDown),
    )
    .unwrap();

    rusteadux::run(model).unwrap();
}

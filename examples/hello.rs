use std::io;

use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute, terminal};

use rusteadux::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rusteadux::{command, App, Command, Message};

struct Model {
    last_key: Option<char>,
}

impl App for Model {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(key_event) = msg.downcast::<KeyEvent>() {
            if let KeyModifiers::CONTROL = key_event.modifiers {
                match key_event.code {
                    KeyCode::Char('c') => return Some(Box::new(command::quit)),
                    _ => return None,
                }
            }

            match key_event.code {
                KeyCode::Char(c) => {
                    self.last_key = Some(c);
                    return None;
                }
                _ => unimplemented!(),
            }
        };

        None
    }

    fn view(&self) {
        let msg = format!("Hello! You pressed: {:?}", self.last_key);

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
    let model = Model { last_key: None };

    execute!(
        io::stdout(),
        terminal::EnterAlternateScreen,
        cursor::Hide,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();

    rusteadux::run(model).unwrap();
}

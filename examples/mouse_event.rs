use std::io;

use crossterm::{
    cursor, execute,
    style::Print,
    terminal::{Clear, ClearType},
};

use rusteadux::{
    command::quit,
    crossterm::event::{MouseEvent, MouseEventKind},
    App, Command, Message,
};

struct Model {
    col: u16,
    row: u16,
}

impl App for Model {
    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Ok(mouse_event) = msg.downcast::<MouseEvent>() {
            if let MouseEventKind::Down(_) = mouse_event.kind {
                return Some(Box::new(quit));
            }
            self.col = mouse_event.column;
            self.row = mouse_event.row;
        }

        None
    }

    fn view(&self) {
        let msg = format!(
            "Click to terminate. Mouse col: {}, row: {}",
            self.col, self.row
        );

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
    let model = Model { col: 0, row: 0 };

    execute!(
        io::stdout(),
        cursor::MoveTo(0, 0),
        Clear(ClearType::FromCursorDown),
    )
    .unwrap();

    rusteadux::enable_mouse_capture().unwrap();
    rusteadux::run(model).unwrap();
}

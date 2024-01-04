use std::io::Result;

use rusteadux::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rusteadux::view_helper::input::Input;
use rusteadux::view_helper::string_printer::StringPrinter;
use rusteadux::{command, App, Command, Message};

struct View {
    printer: StringPrinter,
}

impl View {
    fn new() -> Self {
        Self {
            printer: StringPrinter::new(),
        }
    }

    fn update(&mut self, input: &Input, name: &Option<String>) {
        let prompt = "Enter your name: ";
        let mut output = format!(
            "{}{}\n{}^",
            prompt,
            input.buffer(),
            " ".repeat(prompt.len() + input.pos())
        );
        output = if let Some(name) = &name {
            format!("{}\nHello, {}!", output, name)
        } else {
            output
        };

        self.printer.update(output);
    }

    fn render(&self) -> Result<()> {
        self.printer.print()?;
        Ok(())
    }
}

struct Model {
    view: View,
    input: Input,
    name: Option<String>,
}

impl App for Model {
    fn init(&self) -> Option<Command> {
        None
    }

    fn update(&mut self, msg: Message) -> Option<Command> {
        if let Some(key_event) = msg.downcast_ref::<KeyEvent>() {
            if let KeyModifiers::CONTROL = key_event.modifiers {
                match key_event.code {
                    KeyCode::Char('c') => return Some(Box::new(command::quit)),
                    _ => return None,
                }
            }

            match key_event.code {
                KeyCode::Enter => {
                    self.name = Some(self.input.buffer());
                    self.input.clear();
                    self.view.update(&self.input, &self.name);
                    return None;
                }
                _ => {
                    self.input.on_key_event(*key_event);
                    self.view.update(&self.input, &self.name);
                    return None;
                }
            }
        };

        None
    }

    fn view(&self) {
        self.view.render().unwrap();
    }
}

fn main() {
    let mut model = Model {
        view: View::new(),
        input: Input::new(),
        name: None,
    };
    model.view.update(&model.input, &model.name);

    rusteadux::run(model).unwrap();
}

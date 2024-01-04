# rusteadux

An easy-to-use TUI crate for Rust, based off of the Elm architecture. This is a
re-implementation of Go's [Tea](https://github.com/tj/go-tea), created by TJ
Holowaychuk.

# History

This project is a continuation of the [rustea
crate](https://crates.io/crates/rustea), originally developed by a now deleted
GitHub user, lazops. More information on that project and my attempts to
restore as much of it as I could can be found in
[frayer/rustea](https://github.com/frayer/rustea).

## Features

- Minimal and easy to use API.
- Growing collection of view helpers.
- Automatically multithreaded command processing.
- Cross-platform, thanks to `crossterm`.
- The praised Elm architecture.

## Installation and Docs

TBD

## Quickstart

An example demonstrating a website length checker, with batched asynchronous commands.

```rust
use std::io;

use crossterm::{
    cursor,
    event::KeyModifiers,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

use rusteadux::{
    command,
    crossterm::event::{KeyCode, KeyEvent},
    view_helper::input::Input,
    App, Command, Message,
};

/// Your model stores and maintains the state of your application.
struct Model {
    url_input: Input,
    website_lengths: Vec<usize>,
}

/// Implementing the `App` trait allows you to define the three core methods of the ELM
/// architecture: `init (optional)` , `update`, `view`.
impl App for Model {
    fn update(&mut self, msg: Message) -> Option<Command> {
        // Check if the msg was a KeyEvent.
        if let Some(key_event) = msg.downcast_ref::<KeyEvent>() {
            // Check if the key event is CTRL + C, and if so, return the quit command.
            if let KeyModifiers::CONTROL = key_event.modifiers {
                if let KeyCode::Char('c') = key_event.code {
                    return Some(Box::new(command::quit));
                }
            }

            match key_event.code {
                // If Enter is pressed, return a batch of commands.
                KeyCode::Enter => {
                    let url = self.url_input.buffer();
                    self.url_input.clear();

                    // Make 3 requests to demonstrate command batching.
                    let commands = vec![
                        make_request_command(url.clone()),
                        make_request_command(url.clone()),
                        make_request_command(url),
                    ];
                    return Some(command::batch(commands));
                }
                // If anything else is pressed, pass the key event to the Input view helper.
                _ => self.url_input.on_key_event(*key_event),
            }
        } else if let Some(len) = msg.downcast_ref::<WebsiteLengthMessage>() {
            // Process the result of the individual `make_request_command` commands which
            // themselves return a WebsiteLengthMessage type.
            self.website_lengths.push(len.0);
        }

        None
    }

    fn view(&self) {
        let out = format!(
            "Website URL (press enter when done): {}",
            self.url_input.buffer()
        );

        // Leverage the `crossterm::execute!` macro to print to the terminal.
        execute!(
            io::stdout(),
            cursor::MoveTo(0, 0),
            Print(out),
            Clear(ClearType::UntilNewLine),
        )
        .unwrap();

        for (i, len) in self.website_lengths.iter().enumerate() {
            execute!(
                io::stdout(),
                cursor::MoveTo(0, 1 + i as u16),
                Print(format!("Hit {} length: {}", i, len)),
                Clear(ClearType::UntilNewLine),
            )
            .unwrap();
        }
    }
}

struct WebsiteLengthMessage(usize);

fn make_request_command(url: String) -> Command {
    Box::new(move || {
        // It's okay to block since commands are multi threaded
        let website_len = reqwest::blocking::get(url).unwrap().bytes().unwrap().len();
        Some(Box::new(WebsiteLengthMessage(website_len)))
    })
}

fn main() {
    // Clear the terminal at the start of the application
    execute!(
        io::stdout(),
        cursor::MoveTo(0, 0),
        Clear(ClearType::FromCursorDown),
    )
    .unwrap();

    // Run the application loop
    rusteadux::run(Model {
        url_input: Input::new(),
        website_lengths: Vec::new(),
    })
    .unwrap();
}
```

### More Examples

See the examples directory for more. They can be run with `cargo run --example <example>`.

e.g. `cargo run --example website_length_checker`

#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]

use std::borrow::Cow;
use std::io::Result;
use std::process::{Child, Command, Stdio};

#[cfg(feature = "async")]
mod spawn_async;
#[cfg(not(feature = "async"))]
mod spawn_blocking;
#[cfg(test)]
mod tests;

type CowString = Cow<'static, str>;

#[derive(Debug)]
pub struct Button {
    button_type: &'static str,
    text: CowString,
    action: Option<CowString>,
}

impl Button {
    fn new(
        button_type: &'static str,
        text: impl Into<CowString>,
        action: Option<CowString>,
    ) -> Self {
        let text = text.into();
        Self {
            button_type,
            text,
            action,
        }
    }

    pub fn simple(text: impl Into<CowString>, action: impl Into<CowString>) -> Self {
        Button::new("-b", text, Some(action.into()))
    }

    pub fn simple_no_terminal(text: impl Into<CowString>, action: impl Into<CowString>) -> Self {
        Button::new("-B", text, Some(action.into()))
    }

    pub fn dismiss(text: impl Into<CowString>, action: impl Into<CowString>) -> Self {
        Button::new("-z", text, Some(action.into()))
    }

    pub fn dismiss_no_terminal(text: impl Into<CowString>, action: impl Into<CowString>) -> Self {
        Button::new("-Z", text, Some(action.into()))
    }

    pub fn override_default_dismiss(text: impl Into<CowString>) -> Self {
        Button::new("-s", text, None)
    }

    pub fn detailed(text: impl Into<CowString>) -> Self {
        Button::new("-L", text, None)
    }
}

#[derive(Debug)]
pub enum Edge {
    Top,
    Bottom,
}

impl From<Edge> for CowString {
    fn from(e: Edge) -> Self {
        Cow::Borrowed(match e {
            Edge::Top => "top",
            Edge::Bottom => "bottom",
        })
    }
}

#[derive(Debug)]
pub struct Swaynag {
    message: CowString,
    detailed_message: Option<CowString>,
    terminal: Option<CowString>,
    args: Vec<CowString>,
    buttons: Vec<Button>,
}

impl Swaynag {
    pub fn new(message: impl Into<CowString>) -> Self {
        Self {
            args: Vec::new(),
            buttons: Vec::new(),
            message: message.into(),
            terminal: None,
            detailed_message: None,
        }
    }

    pub fn terminal(mut self, terminal: impl Into<CowString>) -> Self {
        self.terminal = Some(terminal.into());
        self
    }

    pub fn detailed_message(mut self, detailed_message: impl Into<CowString>) -> Self {
        self.detailed_message = Some(detailed_message.into());
        self
    }

    fn arg(&mut self, arg: impl Into<CowString>) {
        self.args.push(arg.into());
    }

    pub fn edge(mut self, edge: Edge) -> Self {
        self.arg("-e");
        self.arg(edge);
        self
    }

    pub fn font(mut self, font: impl Into<CowString>) -> Self {
        self.arg("-f");
        self.arg(font);
        self
    }

    pub fn message_type(mut self, message_type: impl Into<CowString>) -> Self {
        self.arg("-t");
        self.arg(message_type);
        self
    }

    pub fn output(mut self, output: impl Into<CowString>) -> Self {
        self.arg("-o");
        self.arg(output);
        self
    }

    pub fn background(mut self, background: impl Into<CowString>) -> Self {
        self.arg("--background");
        self.arg(background);
        self
    }

    pub fn border(mut self, border: impl Into<CowString>) -> Self {
        self.arg("--border");
        self.arg(border);
        self
    }

    pub fn border_bottom(mut self, border_bottom: impl Into<CowString>) -> Self {
        self.arg("--border-bottom");
        self.arg(border_bottom);
        self
    }

    pub fn button_background(mut self, button_background: impl Into<CowString>) -> Self {
        self.arg("--button-background");
        self.arg(button_background);
        self
    }

    pub fn text(mut self, text: impl Into<CowString>) -> Self {
        self.arg("--text");
        self.arg(text);
        self
    }

    pub fn button_text(mut self, button_text: impl Into<CowString>) -> Self {
        self.arg("--button-text");
        self.arg(button_text);
        self
    }

    pub fn border_bottom_size(mut self, border_bottom_size: impl Into<CowString>) -> Self {
        self.arg("--border-bottom-size");
        self.arg(border_bottom_size);
        self
    }

    pub fn message_padding(mut self, message_padding: impl Into<CowString>) -> Self {
        self.arg("--message-padding");
        self.arg(message_padding);
        self
    }

    pub fn details_background(mut self, details_background: impl Into<CowString>) -> Self {
        self.arg("--details-background");
        self.arg(details_background);
        self
    }

    pub fn details_border_size(mut self, details_border_size: impl Into<CowString>) -> Self {
        self.arg("--details-border-siz");
        self.arg(details_border_size);
        self
    }

    pub fn button_border_size(mut self, button_border_size: impl Into<CowString>) -> Self {
        self.arg("--button-border-size");
        self.arg(button_border_size);
        self
    }

    pub fn button_gap(mut self, button_gap: impl Into<CowString>) -> Self {
        self.arg("--button-gap");
        self.arg(button_gap);
        self
    }

    pub fn button_dismiss_gap(mut self, button_dismiss_gap: impl Into<CowString>) -> Self {
        self.arg("--button-dismiss-gap");
        self.arg(button_dismiss_gap);
        self
    }

    pub fn button_margin_right(mut self, button_margin_right: impl Into<CowString>) -> Self {
        self.arg("--button-margin-right");
        self.arg(button_margin_right);
        self
    }

    pub fn button_padding(mut self, button_padding: impl Into<CowString>) -> Self {
        self.arg("--button-padding");
        self.arg(button_padding);
        self
    }

    pub fn button(mut self, button: Button) -> Self {
        self.buttons.push(button);
        self
    }

    pub fn buttons(mut self, buttons: impl IntoIterator<Item = Button>) -> Self {
        for button in buttons {
            self.buttons.push(button);
        }
        self
    }

    fn spawn_child(&self) -> Result<Child> {
        let mut cmd = Command::new("swaynag");
        cmd.stdout(Stdio::null());
        cmd.stderr(Stdio::null());
        cmd.arg("-m");
        cmd.arg(self.message.as_ref());
        if let Some(ref terminal) = self.terminal {
            cmd.env("TERMINAL", terminal.as_ref());
        }
        cmd.args(self.args.iter().map(|e| e.as_ref()));
        for Button {
            button_type,
            text,
            action,
        } in self.buttons.iter()
        {
            cmd.arg(button_type);
            cmd.arg(text.as_ref());
            if let Some(action) = action {
                cmd.arg(action.as_ref());
            }
        }
        if self.detailed_message.is_some() {
            cmd.arg("-l");
            cmd.stdin(Stdio::piped());
        }
        cmd.spawn()
    }
}

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
    button_type: CowString,
    text: CowString,
    action: Option<CowString>,
}

impl Button {
    fn new(
        button_type: impl Into<CowString>,
        text: impl Into<CowString>,
        action: Option<CowString>,
    ) -> Self {
        let button_type = button_type.into();
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
}

impl Swaynag {
    pub fn new(message: impl Into<CowString>) -> Self {
        Self {
            message: message.into(),
            detailed_message: None,
            terminal: None,
            args: Vec::new(),
        }
    }

    pub fn terminal(&mut self, terminal: impl Into<CowString>) -> &mut Self {
        self.terminal = Some(terminal.into());
        self
    }

    pub fn detailed_message(&mut self, detailed_message: impl Into<CowString>) -> &mut Self {
        self.detailed_message = Some(detailed_message.into());
        self
    }

    fn arg(&mut self, arg: impl Into<CowString>) -> &mut Self {
        self.args.push(arg.into());
        self
    }

    pub fn edge(&mut self, edge: Edge) -> &mut Self {
        self.arg("-e").arg(edge)
    }

    pub fn font(&mut self, font: impl Into<CowString>) -> &mut Self {
        self.arg("-f").arg(font)
    }

    pub fn message_type(&mut self, message_type: impl Into<CowString>) -> &mut Self {
        self.arg("-t").arg(message_type)
    }

    pub fn output(&mut self, output: impl Into<CowString>) -> &mut Self {
        self.arg("-o").arg(output)
    }

    pub fn background(&mut self, background: impl Into<CowString>) -> &mut Self {
        self.arg("--background").arg(background)
    }

    pub fn border(&mut self, border: impl Into<CowString>) -> &mut Self {
        self.arg("--border").arg(border)
    }

    pub fn border_bottom(&mut self, border_bottom: impl Into<CowString>) -> &mut Self {
        self.arg("--border-bottom").arg(border_bottom)
    }

    pub fn button_background(&mut self, button_background: impl Into<CowString>) -> &mut Self {
        self.arg("--button-background").arg(button_background)
    }

    pub fn text(&mut self, text: impl Into<CowString>) -> &mut Self {
        self.arg("--text").arg(text)
    }

    pub fn button_text(&mut self, button_text: impl Into<CowString>) -> &mut Self {
        self.arg("--button-text").arg(button_text)
    }

    pub fn border_bottom_size(&mut self, border_bottom_size: impl Into<CowString>) -> &mut Self {
        self.arg("--border-bottom-size").arg(border_bottom_size)
    }

    pub fn message_padding(&mut self, message_padding: impl Into<CowString>) -> &mut Self {
        self.arg("--message-padding").arg(message_padding)
    }

    pub fn details_background(&mut self, details_background: impl Into<CowString>) -> &mut Self {
        self.arg("--details-background").arg(details_background)
    }

    pub fn details_border_size(&mut self, details_border_size: impl Into<CowString>) -> &mut Self {
        self.arg("--details-border-size").arg(details_border_size)
    }

    pub fn button_border_size(&mut self, button_border_size: impl Into<CowString>) -> &mut Self {
        self.arg("--button-border-size").arg(button_border_size)
    }

    pub fn button_gap(&mut self, button_gap: impl Into<CowString>) -> &mut Self {
        self.arg("--button-gap").arg(button_gap)
    }

    pub fn button_dismiss_gap(&mut self, button_dismiss_gap: impl Into<CowString>) -> &mut Self {
        self.arg("--button-dismiss-gap").arg(button_dismiss_gap)
    }

    pub fn button_margin_right(&mut self, button_margin_right: impl Into<CowString>) -> &mut Self {
        self.arg("--button-margin-right").arg(button_margin_right)
    }

    pub fn button_padding(&mut self, button_padding: impl Into<CowString>) -> &mut Self {
        self.arg("--button-padding").arg(button_padding)
    }

    pub fn button(
        &mut self,
        Button {
            button_type,
            text,
            action,
        }: Button,
    ) -> &mut Self {
        self.args.push(button_type);
        self.args.push(text);
        if let Some(action) = action {
            self.args.push(action);
        }
        self
    }

    pub fn buttons(&mut self, buttons: impl IntoIterator<Item = Button>) -> &mut Self {
        for button in buttons {
            self.button(button);
        }
        self
    }

    fn spawn_child(&self) -> Result<Child> {
        let mut cmd = Command::new("swaynag");
        if let Some(ref terminal) = self.terminal {
            cmd.env("TERMINAL", terminal.as_ref());
        }
        if self.detailed_message.is_some() {
            cmd.arg("-l").stdin(Stdio::piped());
        }
        cmd.stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg("-m")
            .arg(self.message.as_ref())
            .args(self.args.iter().map(|a| a.as_ref()))
            .spawn()
    }
}

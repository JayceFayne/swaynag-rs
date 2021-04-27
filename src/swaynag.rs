use crate::{Child, CowString};
use std::io::Error;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct Swaynag {
    message: CowString,
    terminal: Option<CowString>,
    args: Vec<CowString>,
}

impl Swaynag {
    pub fn new(message: impl Into<CowString>) -> Self {
        Self {
            message: message.into(),
            terminal: None,
            args: Vec::new(),
        }
    }

    pub fn terminal(&mut self, terminal: impl Into<CowString>) -> &mut Self {
        self.terminal = Some(terminal.into());
        self
    }

    fn arg(&mut self, arg: impl Into<CowString>) -> &mut Self {
        self.args.push(arg.into());
        self
    }

    pub fn display_on_bottom_edge(&mut self) -> &mut Self {
        self.arg("-e").arg("bottom")
    }

    pub fn display_on_top_edge(&mut self) -> &mut Self {
        self.arg("-e").arg("top")
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
        text: impl Into<CowString>,
        action: impl Into<CowString>,
    ) -> &mut Self {
        self.arg("-b").arg(text).arg(action)
    }

    pub fn button_no_terminal(
        &mut self,
        text: impl Into<CowString>,
        action: impl Into<CowString>,
    ) -> &mut Self {
        self.arg("-B").arg(text).arg(action)
    }

    pub fn dismiss_button(
        &mut self,
        text: impl Into<CowString>,
        action: impl Into<CowString>,
    ) -> &mut Self {
        self.arg("-z").arg(text).arg(action)
    }

    pub fn dismiss_button_no_terminal(
        &mut self,
        text: impl Into<CowString>,
        action: impl Into<CowString>,
    ) -> &mut Self {
        self.arg("-Z").arg(text).arg(action)
    }

    pub fn override_default_dismiss_button(&mut self, text: impl Into<CowString>) -> &mut Self {
        self.arg("-s").arg(text)
    }

    pub fn details_button(&mut self, text: impl Into<CowString>) -> &mut Self {
        self.arg("-L").arg(text)
    }

    pub fn spawn(&self) -> Result<Child, Error> {
        Ok(Child::new(Command::from(self).spawn()?))
    }
}

#[cfg(not(feature = "async"))]
mod spawn {
    use super::*;
    use std::io::{self, Read, Write};

    impl Swaynag {
        pub fn spawn_with_detailed_message(
            &self,
            detailed_message: impl Into<CowString>,
        ) -> Result<Child, Error> {
            let mut child = Command::from(self)
                .arg("-l")
                .stdin(Stdio::piped())
                .spawn()?;
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(detailed_message.into().as_bytes())?;
            }
            Ok(Child::new(child))
        }

        pub fn spawn_with_detailed_message_reader<T>(
            &self,
            detailed_message: &mut T,
        ) -> Result<Child, Error>
        where
            T: ?Sized + Read,
        {
            let mut child = Command::from(self)
                .arg("-l")
                .stdin(Stdio::piped())
                .spawn()?;
            if let Some(ref mut stdin) = child.stdin.take() {
                io::copy(detailed_message, stdin)?;
            }
            Ok(Child::new(child))
        }
    }
}

#[cfg(feature = "async")]
mod spawn {
    use super::*;
    use async_io::Async;
    use futures_lite::io::{self, AsyncRead, AsyncWriteExt};
    use std::marker::Unpin;

    impl Swaynag {
        pub async fn spawn_with_detailed_message(
            &self,
            detailed_message: impl Into<CowString>,
        ) -> Result<Child, Error> {
            let mut child = Command::from(self)
                .arg("-l")
                .stdin(Stdio::piped())
                .spawn()?;
            if let Some(stdin) = child.stdin.take() {
                Async::new(stdin)?
                    .write_all(detailed_message.into().as_bytes())
                    .await?;
            }
            Ok(Child::new(child))
        }

        pub async fn spawn_with_detailed_message_reader<T>(
            &self,
            detailed_message: T,
        ) -> Result<Child, Error>
        where
            T: AsyncRead + Unpin,
        {
            let mut child = Command::from(self)
                .arg("-l")
                .stdin(Stdio::piped())
                .spawn()?;
            if let Some(stdin) = child.stdin.take() {
                io::copy(detailed_message, Async::new(stdin)?).await?;
            }
            Ok(Child::new(child))
        }
    }
}

impl From<&Swaynag> for Command {
    fn from(swaynag: &Swaynag) -> Self {
        let mut cmd = Command::new("swaynag");
        if let Some(ref terminal) = swaynag.terminal {
            cmd.env("TERMINAL", terminal.as_ref());
        }
        cmd.stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg("-m")
            .arg(swaynag.message.as_ref())
            .args(swaynag.args.iter().map(|a| a.as_ref()));
        cmd
    }
}

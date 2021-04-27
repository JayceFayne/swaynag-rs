use crate::{Child, CowString, Swaynag};
use std::io::Error;
use std::io::{self, Read, Write};
use std::process::{Command, ExitStatus, Stdio};

impl Child {
    pub fn wait(&mut self) -> Result<ExitStatus, Error> {
        self.0.wait()
    }
}

impl Swaynag {
    pub fn spawn(&self) -> Result<Child, Error> {
        Ok(Child(Command::from(self).spawn()?))
    }

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
        Ok(Child(child))
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
        Ok(Child(child))
    }
}

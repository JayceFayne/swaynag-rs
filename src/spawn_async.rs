use crate::{Child, CowString, Swaynag};
use async_io::Async;
use async_pidfd::AsyncPidFd;
use futures_lite::io::{self, AsyncRead, AsyncWriteExt};
use std::io::Error;
use std::marker::Unpin;
use std::process::{Command, ExitStatus, Stdio};

impl Child {
    pub async fn wait(&mut self) -> Result<ExitStatus, Error> {
        Ok(AsyncPidFd::from_pid(self.0.id() as i32)?
            .wait()
            .await?
            .status())
    }
}

impl Swaynag {
    pub fn spawn(&self) -> Result<Child, Error> {
        Ok(Child(Command::from(self).spawn()?))
    }

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
        Ok(Child(child))
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
        Ok(Child(child))
    }
}

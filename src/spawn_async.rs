use crate::Swaynag;
use async_io::Async;
use async_pidfd::AsyncPidFd;
use futures_lite::io::AsyncWriteExt;
use std::io::Result;
use std::process::{Child as ChildProcess, ExitStatus};

pub struct Child(ChildProcess);

impl Child {
    pub fn kill(&mut self) -> Result<()> {
        self.0.kill()
    }

    pub async fn wait(&mut self) -> Result<ExitStatus> {
        Ok(AsyncPidFd::from_pid(self.0.id() as i32)?
            .wait()
            .await?
            .status())
    }
}

impl Swaynag {
    pub async fn spawn(&self) -> std::io::Result<Child> {
        let mut child = self.spawn_child()?;
        if let Some(stdin) = child.stdin.take() {
            if let Some(ref detailed_message) = self.detailed_message {
                Async::new(stdin)?
                    .write_all(detailed_message.as_bytes())
                    .await?;
            }
        }
        Ok(Child(child))
    }
}

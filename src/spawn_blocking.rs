use crate::Swaynag;
use std::io::Result;
use std::io::Write;
use std::process::{Child as ChildProcess, ExitStatus};

pub struct Child(ChildProcess);

impl Child {
    pub fn kill(&mut self) -> Result<()> {
        self.0.kill()
    }

    pub fn wait(&mut self) -> Result<ExitStatus> {
        self.0.wait()
    }
}

impl Swaynag {
    pub fn spawn(&self) -> std::io::Result<Child> {
        let mut child = self.spawn_child()?;
        if let Some(mut stdin) = child.stdin.take() {
            if let Some(ref detailed_message) = self.detailed_message {
                stdin.write_all(detailed_message.as_bytes())?;
            }
        }
        Ok(Child(child))
    }
}

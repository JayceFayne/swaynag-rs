use std::io::Error;
use std::process::{Child as ChildProcess, ExitStatus};

#[derive(Debug)]
pub struct Child(ChildProcess);

impl Child {
    pub(crate) fn new(child: ChildProcess) -> Child {
        Self(child)
    }

    pub fn kill(&mut self) -> Result<(), Error> {
        self.0.kill()
    }
}

#[cfg(not(feature = "async"))]
mod spawn {
    use super::*;

    impl Child {
        pub fn wait(&mut self) -> Result<ExitStatus, Error> {
            self.0.wait()
        }
    }
}

#[cfg(feature = "async")]
mod spawn {
    use super::*;
    use async_pidfd::AsyncPidFd;

    impl Child {
        pub async fn wait(&mut self) -> Result<ExitStatus, Error> {
            Ok(AsyncPidFd::from_pid(self.0.id() as i32)?
                .wait()
                .await?
                .status())
        }
    }
}

use args::Args;
use flags::Flags;
use parking_lot::{lock_api::RwLockReadGuard, FairMutex, RawRwLock, RwLock};
use std::{
    collections::HashMap,
    process::{Child, ChildStderr, ChildStdin, ChildStdout, Stdio},
    sync::Arc,
};
mod system_child_io;
use crate::error::SystemError;
use system_child_io::ReadSystemChildIO;
mod args;
mod error;
mod flags;
/// diffrent types of children one has the IO and one has not so if you wanna fuck with the prosses it is slower than just reading stdin or stdout or stderr
struct SystemChild(Child);
struct SystemChildIO {
    stdin: Option<ChildStdin>,
    stout: Option<ChildStdout>,
    stderr: Option<ChildStderr>,
}
pub struct SystemManager {
    running_childprosesses: Arc<FairMutex<HashMap<SystemCommand, SystemChild>>>,
    running_child_io: Arc<RwLock<HashMap<SystemCommand, RwLock<SystemChildIO>>>>,
}

impl SystemManager {
    fn execute(&mut self, syscommand: &SystemCommand) -> Result<(), error::SystemError> {
        use std::process::Command;
        if self.running_childprosesses.lock().contains_key(&syscommand) {
            return Err(error::SystemError::AlreadySpawned);
        }
        let mut child = Command::new(syscommand.path.clone())
            .args(syscommand.args.clone())
            .args(syscommand.flags.clone())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|v| SystemError::IoError(v))?;
        let child_io = SystemChildIO {
            stdin: child.stdin.take(),
            stout: child.stdout.take(),
            stderr: child.stderr.take(),
        };
        self.running_childprosesses
            .lock()
            .insert(syscommand.clone(), SystemChild(child));
        self.running_child_io
            .write()
            .insert(syscommand.clone(), RwLock::new(child_io));
        Ok(())
    }
    pub fn read_child_io<'a>(&'a self, systemcmd: &SystemCommand) -> Option<ReadSystemChildIO> {
        let lock: RwLockReadGuard<'a, RawRwLock, HashMap<SystemCommand, RwLock<SystemChildIO>>> =
            self.running_child_io.read();
        lock.get(systemcmd).map(move |v| v.read())
    }
}
type Path = String;
#[derive(Eq, Hash, PartialEq, Clone)]
struct SystemCommand {
    path: Path,
    args: Args,
    flags: Flags,
}

trait IntoSystemCommand {
    fn into(self) -> SystemCommand;
}
impl IntoSystemCommand for SystemCommand {
    fn into(self) -> SystemCommand {
        self
    }
}
impl<P: Into<Path>, A: Into<Args>, F: Into<Flags>> IntoSystemCommand for (P, A, F) {
    fn into(self) -> SystemCommand {
        SystemCommand {
            path: self.0.into(),
            args: self.1.into(),
            flags: self.2.into(),
        }
    }
}

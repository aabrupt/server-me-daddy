use args::Args;
use flags::Flags;
use parking_lot::{lock_api::RwLockReadGuard, FairMutex, RawRwLock, RwLock};
use std::{
    collections::HashMap,
    process::{Child, ChildStderr, ChildStdin, ChildStdout, Stdio},
    rc::Rc,
    sync::Arc,
};
use system_child_io::SystemChildIO;
mod system_child_io;
use crate::error::SystemError;

mod args;
mod error;
mod flags;
/// diffrent types of children one has the IO and one has not so if you wanna fuck with the prosses it is slower than just reading stdin or stdout or stderr
struct SystemChild(Child);
pub struct SystemManager {
    running_childprosesses: Arc<RwLock<HashMap<SystemCommand, Rc<RwLock<SystemChild>>>>>,
}

impl SystemManager {
    fn execute(&mut self, syscommand: &SystemCommand) -> Result<(), error::SystemError> {
        use std::process::Command;
        if self.running_childprosesses.read().contains_key(&syscommand) {
            return Err(error::SystemError::AlreadySpawned);
        }
        let child = Command::new(syscommand.path.clone())
            .args(syscommand.args.clone())
            .args(syscommand.flags.clone())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|v| SystemError::IoError(v))?;

        self.running_childprosesses
            .write()
            .insert(syscommand.clone(), Rc::new(RwLock::new(SystemChild(child))));
        Ok(())
    }
    pub fn read_child_io(&self, systemcmd: &SystemCommand) -> Option<SystemChildIO> {
        self.running_childprosesses
            .read()
            .get(systemcmd)
            .map(SystemChildIO::new)
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

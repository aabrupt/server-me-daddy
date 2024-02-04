#[allow(dead_code)]
use args::Args;
use flags::Flags;
use parking_lot::RwLock;
use std::{
    collections::HashMap,
    fs::File,
    hash::{BuildHasher, BuildHasherDefault, DefaultHasher, Hash, Hasher},
    os::fd::{AsRawFd, FromRawFd},
    process::{Child, Stdio},
    rc::Rc,
    sync::Arc,
};
use system_child_io::SystemChildIO;

use crate::error::SystemError;

mod system_child_io;

mod args;
mod error;
mod flags;
#[derive(Clone, Default)]
pub struct SystemManagerSettings<H: BuildHasher = BuildHasherDefault<DefaultHasher>> {
    ///defaults to `/var/tmp/systemmanager`
    path_to_temp_dir: Option<Path>,
    ///hasher to keep state
    hasher: H,
}

struct InnerSystemManagerSettings<H: BuildHasher = BuildHasherDefault<DefaultHasher>> {
    temp_dir_path: Path,
    hasher: H,
}
impl<H: BuildHasher + Default> Default for InnerSystemManagerSettings<H> {
    fn default() -> Self {
        Self {
            temp_dir_path: Default::default(),
            hasher: Default::default(),
        }
    }
}
impl<H: BuildHasher> From<SystemManagerSettings<H>> for InnerSystemManagerSettings<H> {
    fn from(value: SystemManagerSettings<H>) -> Self {
        Self {
            temp_dir_path: value
                .path_to_temp_dir
                .unwrap_or(Path::from("/tmp/systemmanager/")),
            hasher: value.hasher,
        }
    }
}

struct SystemChild(RwLock<Option<Child>>);
pub struct SystemManager<H: BuildHasher = BuildHasherDefault<DefaultHasher>> {
    running_childprosesses: Arc<RwLock<HashMap<SystemCommand, Arc<SystemChild>>>>,
    settings: InnerSystemManagerSettings<H>,
}
impl<H> Default for SystemManager<H>
where
    H: Default + BuildHasher,
{
    fn default() -> Self {
        Self {
            running_childprosesses: Default::default(),
            settings: Default::default(),
        }
    }
}

impl SystemManager<BuildHasherDefault<DefaultHasher>> {
    pub fn new() -> Self {
        Default::default()
    }
}
impl<H: BuildHasher> SystemManager<H> {
    #[cfg(debug_assertions)]
    fn recover_from_restart_or_default() -> HashMap<SystemCommand, Arc<SystemChild>> {
        HashMap::new()
    }

    #[cfg(not(debug_assertions))]
    fn recover_from_restart_or_default() -> HashMap<SystemCommand, Rc<RwLock<SystemChild>>> {
        compile_error!("the recovering is not implemented yet and this should not be relied upon")
    }
    pub fn execute(&mut self, syscommand: &SystemCommand) -> Result<(), error::SystemError> {
        use std::process::Command;
        if self.running_childprosesses.read().contains_key(&syscommand) {
            return Err(error::SystemError::AlreadySpawned);
        }
        let mut hasher = self.settings.hasher.build_hasher();
        syscommand.hash(&mut hasher);
        let stdin = File::create_new(format!(
            "{}/{}.stdin",
            self.settings.temp_dir_path,
            hasher.finish()
        ))
        .map_err(|v| SystemError::IoError(v))?;
        let mut hasher = self.settings.hasher.build_hasher();
        syscommand.hash(&mut hasher);
        let stdout = File::create_new(format!(
            "{}/{}.stdout",
            self.settings.temp_dir_path,
            hasher.finish()
        ))
        .map_err(|v| SystemError::IoError(v))?;

        let mut hasher = self.settings.hasher.build_hasher();
        syscommand.hash(&mut hasher);
        let stderr = File::create_new(format!(
            "{}/{}.stderr",
            self.settings.temp_dir_path,
            hasher.finish()
        ))
        .map_err(|v| SystemError::IoError(v))?;

        let child = Command::new(syscommand.path.clone())
            .args(syscommand.args.clone())
            .args(syscommand.flags.clone())
            .stderr(unsafe { Stdio::from_raw_fd(stderr.as_raw_fd()) })
            .stdout(unsafe { Stdio::from_raw_fd(stdout.as_raw_fd()) })
            .stdin(unsafe { Stdio::from_raw_fd(stdin.as_raw_fd()) })
            .spawn()
            .map_err(|v| SystemError::IoError(v))?;

        self.running_childprosesses.write().insert(
            syscommand.clone(),
            Arc::new(SystemChild(RwLock::new(Some(child)))),
        );
        Ok(())
    }
    pub fn read_child_io(&self, syscommand: &SystemCommand) -> Option<SystemChildIO> {
        self.running_childprosesses
            .read()
            .contains_key(&syscommand)
            .then(|| {
                let mut hasher = self.settings.hasher.build_hasher();
                syscommand.hash(&mut hasher);
                let inner_hash = hasher.finish();
                SystemChildIO::new(inner_hash, self.settings.temp_dir_path.clone())
            })
    }
    pub fn kill(&self, syscommand: &SystemCommand) {
        if !self.running_childprosesses.read().contains_key(&syscommand) {
            return;
        }
        let mut lock = self.running_childprosesses.write();
        let system_child = lock
            .remove(&syscommand)
            .expect("we have already checked it exists and took the lock right after"); // we could just return here it has been removed somehow
        system_child.0.write().take().map(|mut c| c.kill());
    }
}
type Path = String;
#[derive(Eq, Hash, PartialEq, Clone)]
pub struct SystemCommand {
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

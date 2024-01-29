use std::{rc::Rc, sync::RwLockReadGuard};

use parking_lot::RwLock;

use crate::SystemChild;

pub struct SystemChildIO {
    inner: Rc<RwLock<SystemChild>>,
}
impl SystemChildIO {
    pub(crate) fn new(rc: &Rc<RwLock<SystemChild>>) -> SystemChildIO {
        SystemChildIO { inner: rc.clone() }
    }
}

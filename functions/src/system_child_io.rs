pub(crate) struct ReadSystemChildIO<'a> {
    outer_lock: RwLockReadGuard<'a, RawRwLock, HashMap<SystemCommand, RwLock<SystemChildIO>>>,
    read: RwLockReadGuard<'a, RawRwLock, SystemChildIO>,
}
impl ReadSystemChildIO<'a>

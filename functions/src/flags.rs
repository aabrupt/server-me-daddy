pub(crate) type Flag = String;
#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Flags {
    flags: Vec<Flag>,
}
impl IntoIterator for Flags {
    type Item = Flag;

    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.flags.into_iter()
    }
}
impl From<()> for Flags {
    fn from(_: ()) -> Self {
        Default::default()
    }
}

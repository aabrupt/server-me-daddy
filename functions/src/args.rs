pub(crate) type Arg = String;
#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Args {
    args: Vec<Arg>,
}
impl IntoIterator for Args {
    type Item = Arg;

    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.args.into_iter()
    }
}
impl From<()> for Args {
    fn from(_: ()) -> Self {
        Default::default()
    }
}

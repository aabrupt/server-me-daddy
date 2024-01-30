use askama::Template;
#[derive(Template)]
#[template(path="index.html")]
pub struct Index {
    msg: String,
}
impl Index {
    pub(crate) fn new(msg: impl Into<String>) -> Self {
        Self {
            msg: msg.into()
        }
    }
}


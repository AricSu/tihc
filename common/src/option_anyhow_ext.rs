use anyhow::Result;

pub trait OptionAnyhowExt<T> {
    fn anyhow_or(self, msg: impl Into<String>) -> Result<T>;
}

impl<T> OptionAnyhowExt<T> for Option<T> {
    fn anyhow_or(self, msg: impl Into<String>) -> Result<T> {
        self.ok_or_else(|| anyhow::anyhow!(msg.into()))
    }
}

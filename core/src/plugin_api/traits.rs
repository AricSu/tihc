pub trait Plugin {
    fn name(&self) -> &str;
    fn register(&mut self);
}

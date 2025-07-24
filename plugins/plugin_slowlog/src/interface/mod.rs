use anyhow::Result;
use crate::application::SlowLogService;

pub struct SlowLogCliHandler<S: SlowLogService> {
    pub service: S,
}

impl<S: SlowLogService> SlowLogCliHandler<S> {
    pub fn run(&self, file_path: &str) -> Result<String> {
        self.service.parse_and_analyze(file_path)
    }
}

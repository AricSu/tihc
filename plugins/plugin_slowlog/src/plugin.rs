
use core::plugin_api::traits::Plugin;

pub struct SlowLogPlugin;

impl Plugin for SlowLogPlugin {
    fn name(&self) -> &str {
        "slowlog"
    }

    fn register(&mut self) {
        // TODO: 注册命令和服务逻辑应由主平台传递 context 时实现
    }
}

use crate::error::TestError;
use nix::sys::signal::{kill, Signal};
use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};
use tracing::info;

/// 向指定进程直接发送信号
pub fn send_signal_to_pid(target_pid: Pid, sig: Signal) -> Result<(), TestError> {
    let nix_pid = nix::unistd::Pid::from_raw(target_pid.as_u32() as i32);
    let _ = kill(nix_pid, sig);
    Ok(())
}

/// 获取日志绝对路径
pub fn get_log_path_by_pid(pid: i32) -> Result<PathBuf, TestError> {
    // 创建系统信息实例
    let mut sys = System::new();
    sys.refresh_processes();

    // 通过 pid 获取进程
    let process = sys
        .process(Pid::from(pid as usize))
        .ok_or_else(|| TestError::ProcessNotFound(format!("Process with PID {} not found", pid)))?;

    // 提取日志参数
    let log_arg = process
        .cmd()
        .windows(2)
        .find(|w| w[0] == "--log-file")
        .map(|w| w[1].to_string())
        .or_else(|| {
            process
                .cmd()
                .iter()
                .find(|s| s.starts_with("--log-file="))
                .map(|s| s.split_once('=').unwrap().1.to_string())
        })
        .ok_or_else(|| TestError::ProcessNotFound("Missing --log-file parameter".into()))?;

    // 获取进程的当前工作目录
    let cwd = process.cwd();

    // 构建绝对路径
    let absolute_log = if Path::new(&log_arg).is_absolute() {
        PathBuf::from(log_arg)
    } else {
        Path::new(&cwd).join(log_arg)
    };

    // 确保路径存在并规范化
    let absolute_log = absolute_log.canonicalize()?;

    info!("Resolved log path: {}", absolute_log.display());
    Ok(absolute_log)
}
/// 检查日志文件并提取端口号
pub fn check_log_pattern(log_path: &Path, pattern: &str) -> Result<u16, TestError> {
    let mut attempts = 10;

    while attempts > 0 {
        if let Ok(content) = fs::read_to_string(log_path) {
            if let Some(line) = content.lines().find(|l| l.contains(pattern)) {
                // 使用正则表达式提取端口号
                let re = regex::Regex::new(r#""\[::\]:(\d+)""#).unwrap();
                if let Some(caps) = re.captures(line) {
                    return caps
                        .get(1)
                        .and_then(|m| m.as_str().parse().ok())
                        .ok_or(TestError::PortNotFound);
                }
            }
        }

        attempts -= 1;
        std::thread::sleep(Duration::from_millis(500));
    }

    Err(TestError::LogPatternNotFound)
}

use nix::sys::signal::Signal;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};
use utils::error::TestError;
use utils::process::{check_log_pattern, get_log_path_by_pid, send_signal_to_pid};

#[test]
fn test_real_br_backup() -> Result<(), TestError> {
    let target_dir = "/Users/aric/Desktop/szp";
    let _dir = Path::new(target_dir);

    // 1. 清理目录
    cleanup_directory(target_dir)?;

    // 2. 启动备份进程
    let mut cmd = Command::new("tiup")
        .args([
            "br:v7.5.1",
            "backup",
            "full",
            "-s",
            "/Users/aric/Desktop/szp/szp_dir",
            "--log-file",
            "./test_full_backup.log",
        ])
        .current_dir(target_dir)
        .stdout(Stdio::piped())
        .spawn()?;

    println!("Launched backup process PID: {:?}", cmd.id());

    // 3. 查找 BR 子进程
    let pid = Pid::from(cmd.id() as usize);
    let mut sys = System::new();
    let mut attempts = 5;
    let br_pid = loop {
        sys.refresh_processes();

        if let Some((child_pid, _process)) = sys
            .processes()
            .iter()
            .find(|(_, p)| p.parent() == Some(pid) && p.name().to_lowercase().contains("br"))
        {
            break *child_pid;
        }

        if attempts == 0 {
            return Err(TestError::ProcessNotFound(format!(
                "No BR process found under PID: {}",
                pid
            )));
        }

        attempts -= 1;
        std::thread::sleep(Duration::from_millis(500));
    };

    // 4. 发送信号
    send_signal_to_pid(br_pid, Signal::SIGUSR1)?;
    println!("Sent SIGUSR1 to BR process: {}", br_pid);

    // 5. 获取日志路径
    let log_path = get_log_path_by_pid(br_pid.as_u32() as i32)?;

    // 6. 验证日志
    let port = check_log_pattern(&log_path, "bound pprof to addr")?;
    println!("Detected pprof port: {}", port);

    // 7. 清理
    cmd.kill()?;
    Ok(())
}

// 添加缺失的目录清理函数
fn cleanup_directory(dir: impl AsRef<Path>) -> Result<(), TestError> {
    let dir = dir.as_ref();
    if dir.exists() {
        fs::remove_dir_all(dir)?;
    }
    fs::create_dir_all(dir)?;
    Ok(())
}

/// gcc check for Linux glibc builds
#[cfg(all(target_os = "linux", target_env = "gnu"))]
pub fn check_gcc() {
    use std::process::Command;
    if Command::new("gcc").arg("--version").output().is_err() {
        // Try to detect OS type for better suggestion
        let os_hint = std::fs::read_to_string("/etc/os-release").unwrap_or_default();
        let suggestion = if os_hint.contains("centos") || os_hint.contains("rhel") || os_hint.contains("rocky") {
            "sudo yum install -y gcc"
        } else if os_hint.contains("ubuntu") || os_hint.contains("debian") {
            "sudo apt-get update && sudo apt-get install -y gcc build-essential"
        } else {
            "Please install gcc using your distribution's package manager."
        };
        eprintln!("\x1b[31m[ERROR]\x1b[0m gcc not found!\nInstall gcc with: {}", suggestion);
        std::process::exit(1);
    }
}

#[cfg(not(all(target_os = "linux", target_env = "gnu")))]
pub fn check_gcc() {}

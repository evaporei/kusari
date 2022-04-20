use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;
use std::process::{Command, Stdio};

fn main() {
    let git_hash_output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(git_hash_output.stdout).unwrap();
    println!("cargo:rustc-env=COMMIT_HASH={}", git_hash);

    let rustc_version_cmd = Command::new("rustc")
        .arg("--version")
        .stdout(Stdio::piped())
        .spawn();

    unsafe {
        let awk_print_2 = Command::new("awk")
            .arg("{print $2}")
            .stdin(Stdio::from_raw_fd(
                rustc_version_cmd.ok().unwrap().stdout.unwrap().as_raw_fd(),
            ))
            .output()
            .ok();
        let rustc_version = String::from_utf8(awk_print_2.unwrap().stdout).unwrap();
        println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_version);
    }
}

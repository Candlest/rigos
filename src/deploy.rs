use std::process::Command;
use std::path::PathBuf;

pub fn deploy() {
    let work_dir = PathBuf::from("pub");
    // 创建命令
    let mut child = Command::new("git")
    .current_dir(&work_dir)
    .arg("add") // 添加参数
    .arg(".")
    .spawn() // 启动子进程
    .expect("failed to add changes");

    child.wait().expect("failed to add changes");

    let mut child = Command::new("git")
    .current_dir(&work_dir)
    .arg("commit") // 添加参数
    .arg("-m")
    .arg("\"rigos deploy\"")
    .spawn() // 启动子进程
    .expect("failed to commit");

    child.wait().expect("failed to commit");

    child = Command::new("git")
    .current_dir(&work_dir)
        .arg("push") // 添加参数
        .spawn() // 启动子进程
        .expect("failed to push to remote");

    // 等待命令执行完成
    let status = child.wait().expect("failed to push to remote");

    // 打印命令的退出状态
    println!("Command finished with status: {}", status);
}

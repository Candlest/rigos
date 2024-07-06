use std::process::Command;
use std::path::PathBuf;

pub fn deploy() {
    let work_dir = PathBuf::from("pub");
    // 创建命令
    let mut child = Command::new("git")
    .current_dir(&work_dir)
        .arg("push") // 添加参数
        .spawn() // 启动子进程
        .expect("failed to execute process");

    // 等待命令执行完成
    let status = child.wait().expect("failed to wait on child");

    // 打印命令的退出状态
    println!("Command finished with status: {}", status);
}
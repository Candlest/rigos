use crate::PATH;
use std::process::Command;
pub fn deploy() {
    // read input
    let mut input = String::new();
    println!("Please enter the commit message:");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let commit_message = input.trim();
    println!("Commit message: {}", commit_message);

    let mut child = Command::new("git")
        .current_dir(PATH.clone())
        .arg("add") // 添加参数
        .arg(".")
        .spawn() // 启动子进程
        .expect("failed to add changes");

    child.wait().expect("failed to add changes");

    let mut child = Command::new("git")
        .current_dir(PATH.clone())
        .arg("commit") // 添加参数
        .arg("-m")
        .arg("rigos deploy")
        .spawn() // 启动子进程
        .expect("failed to commit");

    child.wait().expect("failed to commit");

    child = Command::new("git")
        .current_dir(PATH.clone())
        .arg("push") // 添加参数
        .spawn() // 启动子进程
        .expect("failed to push to remote");

    // 等待命令执行完成
    let status = child.wait().expect("failed to push to remote");

    // 打印命令的退出状态
    println!("Command finished with status: {}", status);
}

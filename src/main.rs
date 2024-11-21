use std::{
    io::{self, Write},
    process::{Command, Stdio},
};

fn main() {
    loop {
        print!(">");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        if input == "exit" {
            println!("Exiting the shell..");
            return;
        }
        let commands: Vec<&str> = input.split('|').map(|cmd| cmd.trim()).collect();
        if commands.len() == 1 {
            excute_single_commands(commands[0]);
        } else {
            excute_piped_command(commands);
        }
    }
}

fn excute_single_commands(command: &str) {
    let mut parts = command.split_whitespace();
    let cmd = parts.next();
    let args: Vec<&str> = parts.collect();
    if let Some(cmd) = cmd {
        match Command::new(cmd).args(args).spawn() {
            Ok(mut child) => {
                child.wait().unwrap();
            }
            Err(e) => {
                eprintln!("Failed to execute command: {}", e)
            }
        }
    }
}

fn excute_piped_command(commands: Vec<&str>) {
    let mut previous_output = None;
    for (i, command) in commands.iter().enumerate() {
        let mut parts = command.split_whitespace();
        let cmd = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        let mut child = if i == 0 {
            Command::new(cmd)
                .args(&args)
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to execute command")
        } else if i == commands.len() - 1 {
            // 最后一个命令
            Command::new(cmd)
                .args(&args)
                .stdin(previous_output.unwrap())
                .spawn()
                .expect("Failed to execute command")
        } else {
            // 中间命令
            Command::new(cmd)
                .args(&args)
                .stdin(previous_output.unwrap())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to execute command")
        };
        previous_output = child.stdout.take();

        // 等待当前命令完成
        if i == commands.len() - 1 {
            child.wait().unwrap();
        }
    }
}

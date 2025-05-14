use std::process::{Child, ChildStderr, Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

use crate::error::ExitOnError;
use std::io::{BufRead, BufReader};
use std::process::ChildStdout;

type CommandStr<'a> = Vec<&'a str>;

fn read_child_stdout_lines(stdout: Option<ChildStdout>) {
    if stdout.is_none() {
        return;
    }
    let reader = BufReader::new(stdout.unwrap());
    println!(
        "{}",
        reader
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn read_child_stderr_lines(stderr: Option<ChildStderr>) {
    if stderr.is_none() {
        return;
    }
    let reader = BufReader::new(stderr.unwrap());
    println!(
        "{}",
        reader
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<String>>()
            .join("\n")
    );
}

struct Task {
    proc: Child,
    completed: bool,
}

impl Task {
    pub fn run(cmd: CommandStr) -> Self {
        println!("Spawning command: {}", cmd.join(" "));
        let proc = Command::new(cmd[0])
            .args(&cmd[1..])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .log("Cannot spawn child process");
        Self {
            proc,
            completed: false,
        }
    }
    pub fn is_completed(&mut self) {
        self.completed = match self.proc.try_wait() {
            Ok(Some(code)) => {
                read_child_stdout_lines(self.proc.stdout.take());
                read_child_stderr_lines(self.proc.stderr.take());
                println!("Process exited with {}", code);
                true
            }
            _ => false,
        }
    }
}

pub struct Runner {
    tasks: Vec<Task>,
}

impl Runner {
    pub fn new(cmds: Vec<CommandStr>) -> Self {
        let mut runner = Self { tasks: Vec::new() };
        for cmd in cmds {
            runner.tasks.push(Task::run(cmd));
        }
        runner
    }
    pub fn wait(mut self) {
        while !self.tasks.is_empty() {
            for t in self.tasks.iter_mut() {
                t.is_completed();
            }

            self.tasks = self.tasks.into_iter().filter(|t| !t.completed).collect();

            sleep(Duration::from_millis(100));
        }
    }
}

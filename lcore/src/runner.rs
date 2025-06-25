use std::num::NonZero;
// SPDX-License-Identifier: MIT
// Copyright (c) 2025 coppamocha
use std::process::{Child, ChildStderr, Command, Stdio, exit};
use std::thread::sleep;
use std::time::Duration;

use crate::error::{ExitOnError, LiebeError};
use crate::slidingvec::SlidingVec;
use std::io::{BufRead, BufReader};
use std::process::ChildStdout;

pub type CommandStr = Vec<String>;

fn read_child_stdout_lines(stdout: Option<ChildStdout>) {
    if stdout.is_none() {
        return;
    }
    let reader = BufReader::new(stdout.unwrap())
        .lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>();
    if !reader.is_empty() {
        println!("{}", reader.join("\n"));
    }
}

fn read_child_stderr_lines(stderr: Option<ChildStderr>) {
    if stderr.is_none() {
        return;
    }
    let reader = BufReader::new(stderr.unwrap())
        .lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>();
    if !reader.is_empty() {
        println!("{}", reader.join("\n"));
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TaskStatus(usize);

impl TaskStatus {
    pub fn get_status(&self) -> &str {
        match self.0 {
            1 => "COMPLETED",
            2 => "ERROR",
            3 => "RUNNING",
            4 => "WAITING",
            _ => unreachable!(),
        }
    }
    pub fn running() -> Self {
        Self(3)
    }
    pub fn completed() -> Self {
        Self(1)
    }
    pub fn error() -> Self {
        Self(2)
    }
    pub fn waiting() -> Self {
        Self(4)
    }
}

#[derive(Debug)]
pub struct Task {
    proc: Option<Child>,
    status: TaskStatus,
    non_fatal: bool,
    cmd: CommandStr,
}

impl Task {
    pub fn new(cmd: CommandStr) -> Self {
        Self {
            proc: None,
            cmd,
            non_fatal: false,
            status: TaskStatus::running(),
        }
    }
    pub fn run(&mut self) {
        let cmd_str = self.cmd.join(" ");
        println!("Spawning command: {}", cmd_str);
        let proc = Command::new(self.cmd[0].clone())
            .args(&self.cmd[1..])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .log(LiebeError::CantSpawnChildProc(&cmd_str));
        self.proc = Some(proc);
    }
    pub fn get_status(&mut self) -> TaskStatus {
        if self.status != TaskStatus::running() {
            return self.status;
        }
        self.status = match self.proc.as_mut().unwrap().try_wait() {
            Ok(Some(code)) => {
                read_child_stdout_lines(self.proc.as_mut().unwrap().stdout.take());
                read_child_stderr_lines(self.proc.as_mut().unwrap().stderr.take());
                println!(
                    "Process `{}` exited with {}",
                    self.cmd.join(" "),
                    code.code().unwrap_or_default()
                );
                TaskStatus::completed()
            }
            Err(e) => {
                if !self.non_fatal {
                    eprintln!("Error in child process: {}", e);
                    exit(1);
                }
                TaskStatus::error()
            }
            _ => TaskStatus::running(),
        };
        self.status
    }
}

pub struct Runner {
    tasks: SlidingVec<Task>,
    max_proc: usize,
}

impl Runner {
    pub fn new() -> Self {
        Runner {
            tasks: SlidingVec::new(),
            max_proc: std::thread::available_parallelism()
                .unwrap_or(NonZero::new(1).unwrap())
                .into(),
        }
    }
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
    pub fn run(&mut self) {
        self.tasks.window_right(self.max_proc, |tasks| {
            for task in tasks.iter_mut() {
                task.run();
            }

            while !tasks.is_empty() {
                let all_done = tasks
                    .iter_mut()
                    .all(|t| t.get_status() == TaskStatus::completed());
                if all_done {
                    break;
                }
                sleep(Duration::from_millis(100));
            }
        });
        self.tasks.pop_n(self.tasks.iter().len());
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2025 coppamocha
use crate::empty_err;
use crate::error::{ExitOnError, LiebeError};
use crate::slidingvec::SlidingVec;
use std::io::{BufRead, BufReader};
use std::num::NonZero;
use std::process::ChildStdout;
use std::process::{Child, ChildStderr, Command, Stdio, exit};
use std::thread::{JoinHandle, sleep};
use std::time::Duration;

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
pub struct TaskStatus(u8);

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
        self.status = TaskStatus::running();
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
                if code.success() {
                    TaskStatus::completed()
                } else {
                    TaskStatus::error()
                }
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
    pub fn wait(&mut self) {
        while self.get_status() != TaskStatus::completed() {
            if self.status == TaskStatus::error() {
                if self.non_fatal {
                    break;
                } else {
                    eprintln!("Task failed with error...");
                    exit(1)
                }
            }
        }
    }
}

pub struct RunnerHandle(Option<JoinHandle<Runner>>);

impl RunnerHandle {
    pub fn wait(&mut self) -> Runner {
        self.0
            .take()
            .log(empty_err!(ThreadAlreadyJoined))
            .join()
            .log(empty_err!(ThreadFailedToJoin))
    }
}

impl Drop for RunnerHandle {
    fn drop(&mut self) {
        match self.0.take() {
            Some(handle) => {
                handle.join().log(empty_err!(ThreadFailedToJoin));
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct Runner {
    tasks: SlidingVec<Task>,
    pub status: TaskStatus,
    pub max_proc: usize,
}

type RunnerStatus = TaskStatus;

impl Runner {
    pub fn new() -> Self {
        Runner {
            tasks: SlidingVec::new(),
            max_proc: std::thread::available_parallelism()
                .unwrap_or(NonZero::new(1).unwrap())
                .into(),
            status: RunnerStatus::waiting(),
        }
    }
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
    fn run_sync(&mut self) {
        self.status = RunnerStatus::running();
        self.tasks.window_right(self.max_proc, |tasks| {
            for task in tasks.iter_mut() {
                task.run();
            }

            while !tasks.is_empty() {
                let all_done = tasks.iter_mut().all(|t| {
                    t.get_status() == TaskStatus::completed()
                        || t.non_fatal && t.status == TaskStatus::error()
                });
                let has_errored = tasks
                    .iter_mut()
                    .all(|t| !t.non_fatal && t.status == TaskStatus::error());
                if all_done {
                    self.status = RunnerStatus::completed();
                    break;
                }
                if has_errored {
                    self.status = RunnerStatus::error();
                    break;
                }
                sleep(Duration::from_millis(20));
            }
        });
        self.tasks.pop_n(self.tasks.iter().len());
    }

    pub fn run(mut self) -> RunnerHandle {
        RunnerHandle(Some(std::thread::spawn(move || {
            self.run_sync();
            self
        })))
    }

    pub fn get_status(&self) -> RunnerStatus {
        self.status
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2025 coppamocha
use anyhow;
use std::error::Error;
use std::fmt::Display;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};

pub static VERBOSE: AtomicBool = AtomicBool::new(false);

pub fn set_verbose(value: bool) {
    VERBOSE.store(value, Ordering::Release);
}

pub fn is_verbose() -> bool {
    VERBOSE.load(Ordering::Acquire)
}

pub trait ExitOnError<T, E: Error> {
    fn log(self, msg: &str) -> T;
}

impl<T, E> ExitOnError<T, E> for anyhow::Result<T, E>
where
    E: Display + Error,
{
    fn log(self, msg: &str) -> T {
        match self {
            Err(e) => {
                if is_verbose() {
                    eprintln!("{}: {}", msg, e);
                } else {
                    eprintln!("{}", msg);
                }
                process::exit(1);
            }
            Ok(v) => v,
        }
    }
}

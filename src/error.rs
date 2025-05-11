// SPDX-License-Identifier: MIT
// Copyright (c) 2025 coppamocha
use std::process;
use std::error::Error;
use std::fmt::Display;
use anyhow;

pub trait ExitOnError<T, E: Error> {
    fn log(self, msg: &str, verbose: bool) -> T;
}

impl<T, E> ExitOnError<T, E> for anyhow::Result<T, E>
where E: Display + Error
{
    fn log(self, msg: &str, verbose: bool) -> T {
        match self {
            Err(e) => {
                if verbose {
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

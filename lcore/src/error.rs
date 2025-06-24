// SPDX-License-Identifier: MIT
// Copyright (c) 2025 coppamocha
use std::error;
use std::fmt::{Debug, Display};
use std::process::exit;
use thiserror::Error;

pub static mut VERBOSE: bool = true;

pub fn set_verbose(val: bool) {
    unsafe { VERBOSE = val }
}

pub fn get_verbose() -> bool {
    unsafe { return VERBOSE }
}

pub trait ExitOnError<T, E, Q>
where
    E: error::Error,
    T: Debug,
    Q: AsRef<str> + Display,
{
    fn log(self, e: LiebeError<Q>) -> T;
}

impl<T, E, Q> ExitOnError<T, E, Q> for Result<T, E>
where
    E: error::Error,
    T: Debug,
    Q: AsRef<str> + Display,
{
    fn log(self, e: LiebeError<Q>) -> T {
        if self.is_err() {
            if unsafe { VERBOSE } {
                eprintln!("{}: {:#?}", e.as_pretty(), self.unwrap_err());
            } else {
                eprintln!("{}", e);
            }
            exit(1);
        }
        self.unwrap()
    }
}

#[macro_export]
macro_rules! empty_err {
    ($variant: ident) => {
        LiebeError::<String>::$variant
    };
}

#[derive(Debug, Error)]
pub enum LiebeError<T = String>
where
    T: AsRef<str> + Display,
{
    #[error("Cannot spawn child process")]
    CantSpawnChildProc(T),
    #[error("Cannot open file")]
    CannotOpenFile(T),
    #[error("Cannot read from file")]
    CannotReadFile(T),
    #[error("Invalid configuration file")]
    InvalidConf,
    #[error("Cannot open lua stdlibs")]
    CantOpenStdLibs,
    #[error("Cannot create lua table")]
    CannotCreateTable,
    #[error("Cannot inject global lua context")]
    CannotInjectContext(T),
    #[error("Lua function not found")]
    FuncNotFound(T),
    #[error("Cannot call lua function")]
    CannotCallFunc(T),
}

impl<T: AsRef<str> + Display> LiebeError<T> {
    pub fn as_pretty(&self) -> String {
        match self {
            Self::CannotOpenFile(str) => format!("{self}: {str}"),
            Self::CannotReadFile(str) => format!("{self}: {str}"),
            Self::CannotInjectContext(str) => format!("{self}: {str}"),
            Self::FuncNotFound(str) => format!("{self}: {str}"),
            Self::CannotCallFunc(str) => format!("{self}: {str}"),
            _ => self.to_string(),
        }
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2025 coppamocha
use std::env;
use std::path::Path;

pub fn search_file_in_dirs(dirs: &[&str], filename: &str) -> Option<String> {
    for dir in dirs {
        let dir = dir.resolve();
        let path = Path::new(&dir).join(filename);
        if path.exists() {
            return Some(path.to_string_lossy().to_string());
        }
    }
    None
}

pub trait ToResolved<T: AsRef<str>, Q: AsRef<str>> {
    fn resolve(self) -> Q;
}

impl ToResolved<&str, String> for &str {
    fn resolve(self) -> String {
        let pwd = env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        let prog = env::current_exe()
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        let resolved = self.replace("$(PWD)", &pwd).replace("$(PROG)", &prog);
        resolved
    }
}

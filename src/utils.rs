// SPDX-License-Identifier: MIT
// Copyright (c) 2025 coppamocha
use std::env;
use std::path::Path;

pub fn search_file_in_dirs(dirs: &[&str], filename: &str) -> Option<String> {
    for dir in dirs {
        let dir = resolve_path(dir);
        let path = Path::new(&dir).join(filename);
        if path.exists() {
            return Some(path.to_string_lossy().to_string());
        }
    }
    None
}

pub fn resolve_path(path: &str) -> String {
    let pwd = env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_default();
    let prog = env::current_exe()
        .map(|p| p.display().to_string())
        .unwrap_or_default();
    let resolved = path.replace("$(PWD)", &pwd).replace("$(PROG)", &prog);
    resolved
}

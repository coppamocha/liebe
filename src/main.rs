// SPDX-License-Identifier: MIT
// Copyright (c) 2025 coppamocha
const CONFIG_PATH: &str = "$(PWD)/liebe.toml";
use liebe::cli;
use liebe::error::set_verbose;
use liebe::luaapi;
use liebe::runner;

fn main() {
    let run = runner::Runner::new(vec![vec!["echo", "hello", "warld"], vec!["sleep", "5"]]);
    run.wait();
    set_verbose(true);
    let mut lua = luaapi::LuaApi::new(CONFIG_PATH);
    let app = cli::Cli::parse();
    lua.invoke();
    app.apply_callbacks(&lua);
}

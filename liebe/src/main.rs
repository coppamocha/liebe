// SPDX-License-Identifier: MIT
// Copyright (c) 2025 coppamocha
const CONFIG_PATH: &str = "$(PWD)/liebe.toml";
use lcore::{
    luaapi,
    runner::{Runner, Task},
};
use liebe::cli;

fn main() {
    let mut lua = luaapi::LuaApi::new(CONFIG_PATH);
    let app = cli::Cli::parse();
    lua.invoke();
    app.apply_callbacks(&lua);

    let mut runner = Runner::new();
    runner.max_proc = 2;
    runner.add_task(Task::new(vec!["time".into(), "sleep".into(), "4".into()]));
    runner.add_task(Task::new(vec!["time".into(), "sleep".into(), "5".into()]));
    runner.add_task(Task::new(vec!["time".into(), "sleep".into(), "6".into()]));
    runner = runner.run().wait();
    println!("{:#?}", runner);
}

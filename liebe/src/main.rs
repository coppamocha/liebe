// SPDX-License-Identifier: MIT
// Copyright (c) 2025 coppamocha
const CONFIG_PATH: &str = "$(PWD)/liebe.toml";
use lcore::luaapi;
use liebe::cli;

fn main() {
    let mut lua = luaapi::LuaApi::new(CONFIG_PATH);
    let app = cli::Cli::parse();
    lua.invoke();
    app.apply_callbacks(&lua);
}

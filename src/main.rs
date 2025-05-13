// SPDX-License-Identifier: MIT
// Copyright (c) 2025 coppamocha
const CONFIG_PATH: &str = "$(PWD)/liebe.toml";
use liebe::cli;
use liebe::error::set_verbose;
use liebe::luaapi;

fn main() {
    set_verbose(true);
    let mut lua = luaapi::LuaApi::new(CONFIG_PATH);
    let app = cli::Cli::parse();
    lua.invoke();
    app.apply_callbacks(&lua);
}

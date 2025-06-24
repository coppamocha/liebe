// SPDX-License-Identifier: MIT
// Copyright (c) 2025 coppamocha
use crate::empty_err;
use crate::error::*;
use crate::utils::{self, *};
use mlua::prelude::*;
use std::fmt::Debug;
use std::fs;
use std::io::Read;
use toml;

const SEARCH_DIRS: &[&str] = &[
    "$(PROG)/scripts/",
    "$(PROG)/extensions/",
    "$(PWD)/",
    "$(PWD)/liebe/",
];

pub struct LuaApi {
    config: toml::Value,
    lua: Lua,
}

impl LuaApi {
    pub fn new(config_path: &str) -> Self {
        let config_path = config_path.resolve();
        let mut file =
            fs::File::open(&config_path).log(LiebeError::CannotOpenFile(config_path.clone()));
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .log(LiebeError::CannotReadFile(config_path));

        let config = toml::from_str(&contents).log(empty_err!(InvalidConf));

        let lua = Lua::new();
        lua.load_std_libs(LuaStdLib::ALL_SAFE)
            .log(empty_err!(CantOpenStdLibs));

        Self { config, lua }
    }

    pub fn invoke(&mut self) {
        let lang_script = self
            .config
            .get("lang-script")
            .expect("Error: expected lang-script field in liebe.toml")
            .as_str()
            .expect("Error: invalid field");
        let mut contents = String::new();
        let mut file = fs::File::open(
            utils::search_file_in_dirs(SEARCH_DIRS, lang_script)
                .expect("Error: couldnt find lang-script file"),
        )
        .log(LiebeError::CannotOpenFile(format!(
            "{} in {:?}",
            lang_script, SEARCH_DIRS
        )));

        file.read_to_string(&mut contents)
            .log(LiebeError::CannotReadFile(lang_script.to_string()));

        self.lua
            .load(contents)
            .exec()
            .expect("Syntax error in lang-script");
    }

    pub fn request_data<G>(&self, func: &str) -> G
    where
        G: FromLuaMulti + Debug,
    {
        let luafn: mlua::Function = self
            .lua
            .globals()
            .get(func)
            .log(LiebeError::FuncNotFound(func.to_string()));
        luafn
            .call::<G>(())
            .log(LiebeError::CannotCallFunc(func.to_string()))
    }

    pub fn add_context(&self, name: &str, data: mlua::Table) {
        self.lua
            .globals()
            .set(name, data)
            .log(LiebeError::CannotInjectContext(name.to_string()));
    }

    pub fn create_table(&self) -> mlua::Table {
        self.lua.create_table().log(empty_err!(CannotCreateTable))
    }
}

use crate::error::ExitOnError;
use crate::utils::{self, *};
use mlua::prelude::*;
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
        let mut file =
            fs::File::open(&resolve_path(config_path)).log("Error opening configuration file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .log("Error reading configuration contents");

        let config = toml::from_str(&contents).log("Invalid configuration file");

        let lua = Lua::new();
        lua.load_std_libs(LuaStdLib::ALL_SAFE)
            .log("Could not open lua stdlibs");

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
        .log("Cannot open lang-script file");

        file.read_to_string(&mut contents)
            .log("Cannot read from lang-script file");

        self.lua
            .load(contents)
            .exec()
            .expect("Syntax error in lang-script");
    }
}

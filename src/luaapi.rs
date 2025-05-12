
use mlua::prelude::*;
use toml;
use std::fs;
use crate::error::ExitOnError;
use std::io::Read;

pub struct LuaApi {
    config: toml::Value,
    lua: Lua
}

impl LuaApi {
    pub fn new(config_path: String) -> Self {
        let mut file = fs::File::open(&config_path).log("Error opening configuration file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).log("Error reading configuration contents");

        let config = toml::from_str(&contents).log("Invalid configuration file");
        
        let lua = Lua::new();
        lua.load_std_libs(LuaStdLib::ALL).log("Could not open lua stdlibs");

        Self {
            config,
            lua,
        }
    }

}



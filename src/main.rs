// SPDX-License-Identifier: MIT
// Copyright (c) 2025 coppamocha

mod error;
use mlua::prelude::*;
use error::ExitOnError;

fn main() {    
    let lua = Lua::new();
    lua.load_std_libs(LuaStdLib::IO).log("Couldn't load lua libs", true);
    lua.load(r#"
            function add(x, y)
                print("x + y = " .. (x+y) );
                return x+y
            end
        "#, ).exec().log("Couldn't load lua chunk", true);
    let add: LuaFunction = lua.globals().get("add").log("Couldn't find add function", true);
    let _res: i32 = add.call((5,10)).log("Couldn't add", true);
}

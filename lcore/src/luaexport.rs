use mlua::{Lua, MultiValue, Table, Value};

pub trait LuaExtension {
    fn register_fn<T>(
        &mut self,
        func: &'static T,
        lua_name: &str,
        lua_module: &str,
    ) -> Result<(), mlua::Error>
    where
        T: Fn(&Lua, MultiValue) -> Result<MultiValue, mlua::Error>;
}

impl LuaExtension for Lua {
    fn register_fn<T>(
        &mut self,
        func: &'static T,
        lua_name: &str,
        lua_module: &str,
    ) -> Result<(), mlua::Error>
    where
        T: Fn(&Lua, MultiValue) -> Result<MultiValue, mlua::Error>,
    {
        let lua_func = self.create_function(|a, b| func(a, b))?;

        let globals = self.globals();

        let module: Table = match globals.get::<Value>(lua_module)? {
            Value::Table(t) => t,
            _ => {
                let t = self.create_table()?;
                globals.set(lua_module, t.clone())?;
                t
            }
        };

        module.set(lua_name, lua_func)?;
        Ok(())
    }
}

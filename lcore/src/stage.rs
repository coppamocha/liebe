// SPDX-License-Identifier: MIT
// Copyright (c) 2025 coppamocha

use crate::runner::CommandStr;
use mlua::{FromLua, Lua, Result, Value};
use mlua::{FromLuaMulti, MultiValue};

macro_rules! impl_lua_userdata_fields {
    ($type:ty, { $( $field:ident ),* $(,)? }) => {
        impl mlua::UserData for $type {
            fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
                $(
                    fields.add_field_method_get(stringify!($field), |_, this| Ok(this.$field.clone()));
                    fields.add_field_method_set(stringify!($field), |_, this, val| {
                        this.$field = val;
                        Ok(())
                    });
                )*
            }
        }
    };
}

#[derive(Debug)]
pub struct BuildStage {
    pub name: String,
    pub commands: Vec<CommandStr>,
}

impl_lua_userdata_fields!(BuildStage, { name, commands });

impl FromLua for BuildStage {
    fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
        match value {
            Value::Table(table) => {
                let name = table.get("name")?;
                let commands = table.get("commands")?;
                Ok(BuildStage { name, commands })
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "BuildStage".to_string(),
                message: Some("expected a table with fields 'name' and 'commands'".into()),
            }),
        }
    }
}

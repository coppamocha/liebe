use clap::{Arg, ArgAction, ArgMatches, Command};
use mlua::Table;

use crate::{error::set_verbose, luaapi::LuaApi};

pub const VERSION: &str = "0.1";

pub struct Cli {
    matches: ArgMatches,
    pub unmatched_args: Vec<String>,
}

impl Cli {
    pub fn parse() -> Self {
        let matches = Command::new("liebe")
            .version(VERSION)
            .author("coppamocha")
            .about("A next-generation build system without a headache")
            .subcommand(
                Command::new("run").about("Build and run the project").arg(
                    Arg::new("target")
                        .help("Target to build (and run). eg- debug, release")
                        .required(true)
                        .index(1),
                ),
            )
            .subcommand(
                Command::new("build").about("Build the project").arg(
                    Arg::new("target")
                        .help("Target to build. eg- debug, release")
                        .required(true)
                        .index(1),
                ),
            )
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .help("Allow a verbose output")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("lua-args")
                    .num_args(0..)
                    .trailing_var_arg(true)
                    .allow_hyphen_values(true),
            )
            .get_matches();
        let unmatched_args = matches
            .get_many::<String>("lua-args")
            .unwrap_or_default()
            .cloned()
            .collect::<Vec<String>>();
        Self {
            unmatched_args,
            matches,
        }
    }

    pub fn apply_callbacks(self, lua: &LuaApi) {
        match self.matches.subcommand() {
            Some(("build", subc)) => {
                let target = subc
                    .get_one::<String>("target")
                    .expect("Expected a target to build");
                Self::on_build(target, lua);
            }
            Some(("run", subc)) => {
                let target = subc
                    .get_one::<String>("target")
                    .expect("Expected a target to build and run");
                Self::on_run(target, lua);
            }
            _ => {}
        }
        set_verbose(self.matches.get_flag("verbose"));
    }

    fn on_build(target: &String, lua: &LuaApi) {
        let context = lua.create_table();
        context
            .set("target", target.to_string())
            .expect("Couldnt set value to build_cfg");
        lua.add_context("build_cfg", context);
        lua.request_data("create_build_command", |table: Table| {});
    }
    fn on_run(target: &String, lua: &LuaApi) {}
}

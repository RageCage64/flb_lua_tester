use std::collections::HashMap;
use std::fs::File;

use mlua::{ToLuaMulti, LuaSerdeExt};
use mlua::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub scripts: Vec<ScriptTest>,
}

pub fn load_config(path: String) -> Config {
    let f = File::open(path).expect("Could not open file.");
    return serde_yaml::from_reader(f).expect("Could not read values.");
}

#[derive(Deserialize, Debug)]
pub struct ScriptTest {
    pub file: String,
    pub call: String,
    pub tests: Vec<TestCase>,
}

#[derive(Deserialize, Debug)]
pub struct TestCase {
    pub name: String,
    pub input: LuaFnInput,
    pub expected: LuaFnOutput,
}

#[derive(Deserialize, Debug)]
pub struct LuaFnInput {
    pub tag: String,
    pub timestamp: String,
    pub record: HashMap<String, FlbRecordValidType>,
}

impl<'lua> ToLuaMulti<'lua> for LuaFnInput {
    fn to_lua_multi(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::MultiValue<'lua>> {
        let mut mv = mlua::MultiValue::new();

        let record = lua.create_table().unwrap();
        for row in self.record.iter() {
            let key = lua.to_value(row.0).unwrap();
            let val = lua.to_value(row.1).unwrap();
            record.set(key, val).unwrap();
        }
        mv.push_front(mlua::Value::Table(record));

        mv.push_front(self.timestamp.to_lua(lua).unwrap());

        mv.push_front(self.tag.to_lua(lua).unwrap());

        return Ok(mv);
    }
}

#[derive(Deserialize, Debug)]
pub struct LuaFnOutput {
    pub code: i64,
    pub timestamp: String,
    pub record: HashMap<String, FlbRecordValidType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum FlbRecordValidType {
    String(String),
    Number(f64),
    Table(HashMap<String, FlbRecordValidType>),
}

impl PartialEq for FlbRecordValidType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Table(l0), Self::Table(r0)) => l0 == r0,
            _ => false,
        }
    }
}

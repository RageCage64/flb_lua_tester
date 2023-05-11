mod config;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use config::config::FlbRecordValidType;
use mlua::prelude::*;

fn main() {
    let mut args = env::args();
    if args.len() == 1 {
        println!("Please provide a config file path.");
        return;
    }
    let path = args.nth(1).unwrap();
    let config = config::config::load_config(path);

    for script in config.scripts {
        let lua = load_script(script.file);
        let f_res = lua.globals().get::<_, mlua::Function>(script.call);
        if f_res.is_err() {
            continue;
        }
        let f = f_res.unwrap();
        for tc in script.tests {
            println!("Running test: {:?}", tc.name);
            let mut test_passed = true;

            let r = f.call::<_, LuaMultiValue>(tc.input);
            let mut mv = r.unwrap();

            let code = i64::from_lua(mv.pop_front().unwrap(), &lua).unwrap();
            if code != tc.expected.code {
                test_passed = false;
                println!("  expected code: {:?}", tc.expected.code);
                println!("  got code: {:?}\n", code);
            }            

            let timestamp = String::from_lua(mv.pop_front().unwrap(), &lua).unwrap();
            if timestamp != tc.expected.timestamp {
                test_passed = false;
                println!("  expected timestamp: {:?}", tc.expected.timestamp);
                println!("  got timestamp: {:?}\n", timestamp);
            }

            let record_value: LuaValue = mv.pop_front().unwrap();
            let record: HashMap<String, FlbRecordValidType> = lua.from_value(record_value).unwrap();
            if record != tc.expected.record {
                test_passed = false;
                println!("  expected record: {:?}", tc.expected.record);
                println!("  got record: {:?}\n", record);
            }

            if test_passed {
                println!("Test Passed\n");
            } else {
                println!("Test Failed\n");
            }
        }
    }
}

fn load_script(script_path: String) -> Lua {
    let lua = Lua::new();
    let mut script_content = "".to_string();
    let file = File::open(script_path).unwrap();
    for line in io::BufReader::new(file).lines() {
        script_content += &line.unwrap();
        script_content += "\n";
    }
    lua.load(&script_content).exec().unwrap();
    return lua;
}

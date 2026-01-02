use std::collections::HashMap;
use regex::Regex;
use serde_json::{json, Value};

pub fn parser_function(llm_result: &str) -> (String, Vec<String>){
    let re = Regex::new(r"<action>(.*?)</action>").expect("Regex compile error");

    let expr = re.captures(llm_result)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().trim())
        .expect("not found the action");
    let (func_name, mut params_part) = expr.split_once("(").expect("split error");
    params_part = params_part.trim_end_matches(")").trim();
    let params = params_part.split(",")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    let func_name = func_name.trim();
    (String::from(func_name), params)
}

type FunctionType = Box<dyn Fn(Vec<&str>) -> Value>;

pub struct Tool{
    tool_call: HashMap<String, FunctionType>,
    description: Value,
}
pub struct ToolList{
    tool_set: Vec<Tool>
}

impl ToolList {
    pub fn new() -> ToolList {
        Self{
            tool_set: Vec::new(),
        }
    }
    pub fn insert_one_tool(&mut self, ){
        
    }
}


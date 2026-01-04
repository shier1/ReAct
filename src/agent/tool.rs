use std::collections::HashMap;
use regex::Regex;
use super::react_template;
use std::fs;
use std::path;

pub fn parser_function(llm_result: &str) -> (String, Vec<String>){
    let re = Regex::new(r#"<action>([\s\S]*?)</action>"#).expect("Regex compile error");

    let expr = re.captures(llm_result)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().trim())
        .expect("not found the action");
    let (func_name, mut params_part) = expr.split_once("(").expect("split error");
    params_part = params_part.trim_end_matches(")").trim();
    let params = params_part.split("@")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().trim_matches('"').to_string())
        .collect();
    let func_name = func_name.trim().trim_matches('"');
    (String::from(func_name), params)
}

pub trait Tool: Send + Sync {
    fn call(&self, params: Vec<String>) -> String;
    fn metadata(&self) -> ToolMetadata;
}

#[derive(Clone)]
pub struct ToolMetadata {
    pub name: String,
    pub description: String,
}

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}


impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register<T: Tool + 'static>(&mut self, tool: T) {
        let metadata = tool.metadata();
        self.tools.insert(metadata.name.clone(), Box::new(tool));
    }

    pub fn get(&self, name: &str) -> Option<&dyn Tool> {
        self.tools.get(name).map(|t| t.as_ref())
    }

    pub fn list_tools(&self) -> String {
        let mut tool_info = String::new();
        for meta_data in self.tools.values().map(|tool| tool.metadata()){
            tool_info += &format!("{}\n", meta_data.description)
        }
        tool_info
    }
}

pub struct WriteFileTool {
    metadata: ToolMetadata,
}

impl WriteFileTool {
    pub fn new() -> Self {
        Self {
            metadata: ToolMetadata {
                name: "write_to_file".to_string(),
                description: "一个工具用于创建或修改某个文件的内容-参数1:文件路径，参数2:写入的内容。".to_string(),
            },
        }
    }
}

impl Tool for WriteFileTool {
    fn call(&self, params: Vec<String>) -> String {
        if params.len() != 2{
            return react_template::complete_observation_template("参数个数错误");
        }
        let path = params[0].clone();
        let content = params[1].clone();

        let path = path::Path::new(&path);

        if let Some(parent) = path.parent(){
            if !parent.exists(){
                if let Err(_) = fs::create_dir_all(parent){
                    return react_template::complete_observation_template("文件夹创建失败");
                }
            }
        }

        match fs::write(path, content){
            Ok(_) => react_template::complete_observation_template("写入成功"),
            Err(_) => react_template::complete_observation_template("文件写入失败")
        }
    }
    fn metadata(&self) -> ToolMetadata {
        self.metadata.clone()
    }
}

pub fn get_all_tools() -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    registry.register(WriteFileTool::new());
    registry
}
use std::io;
use crate::agent::tool::ToolRegistry;
use super::deepseek_client;
use super::tool;

pub struct ReActAgent{
    model_name: Option<String>,
    llm_client: deepseek_client::DeepSeekClient,
    tool_registry: ToolRegistry,
    message_history: Vec<deepseek_client::ChatMessage>
}

impl ReActAgent{
    pub fn new(model_name: &str, api_key: &str) -> anyhow::Result<Self> {
        if api_key.is_empty(){
            anyhow::bail!("API key is empty");
        }
        Ok(Self{
            model_name: Some(model_name.to_string()),
            llm_client: deepseek_client::DeepSeekClient::new(api_key),
            tool_registry: tool::get_all_tools(),
            message_history: Vec::new(),
        })
    }
    
    pub async fn run(&mut self){
        self.message_history.push(deepseek_client::ChatMessage::system(self.tool_registry.list_tools().as_str()));
        loop {
            let mut user_question = String::new();
            println!("请输入你的指令：");
            io::stdin().read_line(&mut user_question).expect("你输入的指令有误");
            if user_question == "exit"{
                break;
            }
            let mut llm_result;
            self.message_history.push(deepseek_client::ChatMessage::user(user_question.as_str()));

            llm_result = self.llm_client.chat(
                self.message_history.clone(),
                self.model_name.clone(),
                Some(0.6),
                None
            ).await.unwrap();
            println!("{}", llm_result);
            self.message_history.push(deepseek_client::ChatMessage::assistant(llm_result.as_str()));

            while !llm_result.contains("<final_answer>") {
                if llm_result.contains("<action>") {
                    let (tool_name, params) = tool::parser_function(llm_result.as_str());
                    println!("{}, {}", tool_name, params.len());
                    let observation_result = if let Some(tool) = self.tool_registry.get(tool_name.as_str()) {
                        tool.call(params)
                    } else {
                        "agent 工具解析出现错误".to_string()
                    };
                    self.message_history.push(deepseek_client::ChatMessage::observation(observation_result.as_str()));
                    for his in self.message_history.iter(){
                        println!("{}", his.content);
                    }
                    llm_result = self.llm_client.chat(
                        self.message_history.clone(),
                        self.model_name.clone(),
                        Some(0.6),
                        None
                    ).await.unwrap();
                    self.message_history.push(deepseek_client::ChatMessage::assistant(llm_result.as_str()));
                    println!("{}", llm_result);
                }
            }
        }
    }
}
use super::deepseek_client;

#[derive(Debug)]
pub struct ReActAgent{
    model_name: Option<String>,
    llm_client: deepseek_client::DeepSeekClient,
}

impl ReActAgent{
    pub fn new(model_name: &str, api_key: &str) -> anyhow::Result<Self> {
        if api_key.is_empty(){
            anyhow::bail!("API key is empty");
        }
        Ok(Self{
            model_name: Some(model_name.to_string()),
            llm_client: deepseek_client::DeepSeekClient::new(api_key),
        })
    }
    
    pub async fn run(&self, user_problem: &str){
        let mut llm_result = self.llm_client.chat(
            vec![
                deepseek_client::ChatMessage::system(),
                deepseek_client::ChatMessage::user(user_problem)
            ],
            self.model_name.clone(),
            Some(0.6),
            None
        ).await.unwrap();
        println!("{}", llm_result);
        while(!llm_result.contains("<final_answer>" )){
            if llm_result.contains("<action>"){
                
            }
        }
    }
}
mod agent;
use tokio;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = "sk-3e6fca6b40e045158f8148d754c72f4d";
    let model_name = "deepseek-chat";
    let mut react_agent = match agent::ReActAgent::new(model_name, api_key){
        Ok(react_agent)  => react_agent,
        Err(_e) => panic!("Error creating react_agent")
    };
    react_agent.run().await;
    Ok(())
}

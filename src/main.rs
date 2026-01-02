mod agent;
use tokio;
use std::io;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = "sk-3e6fca6b40e045158f8148d754c72f4d";
    let model_name = "deepseek-chat";
    let react_agent = match agent::ReActAgent::new(model_name, api_key){
        Ok(react_agent)  => react_agent,
        Err(_e) => panic!("Error creating react_agent")
    };
    loop{
        let mut user_question = String::new();
        io::stdin().read_line(&mut user_question).unwrap();
        if user_question == "exit"{
            break;
        }
        react_agent.run(user_question.as_str()).await;
    }
}


use serde::{Deserialize, Serialize};
use Role::{Assistant, System, User};
const MODEL: &str = "gpt-3.5-turbo";

#[derive(Serialize, Deserialize)]
pub struct Message {
    role: Role,
    content: String,
}

impl Message {
    pub fn new(role: Role, content: String) -> Self {
        Message { role, content }
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}



#[derive(Serialize)]
pub struct OpenAIRequest {
    pub model: String,
    stream: bool,
    messages: Vec<Message>,
}

impl OpenAIRequest {
    pub fn new(model: String, stream: bool, messages: Vec<Message>) -> Self {
        OpenAIRequest {
            model,
            stream,
            messages,
        }
    }
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn system_message(&mut self, message: String) {
        self.add_message(Message::new(System, message));
    }

    pub fn user_message(&mut self, message: String) {
        self.add_message(Message::new(User, message));
    }

    pub fn assistant_message(&mut self, message: String) {
        self.add_message(Message::new(Assistant, message));
    }
}

impl Default for OpenAIRequest {
    fn default() -> Self {
        OpenAIRequest {
            model: MODEL.to_string(),
            stream: true,
            messages: vec![],
        }
    }
}






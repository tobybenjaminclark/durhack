use std::collections::HashMap;

#[derive(Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Clone)]
pub struct CharacterSession {
    pub name: String,
    pub persona: String,
    pub history: Vec<Message>,
}

pub struct ChatManager {
    pub sessions: HashMap<String, CharacterSession>,
    pub active: Option<String>,
    pub model: String,
}

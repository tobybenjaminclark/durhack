use std::collections::HashMap;
use async_openai::types::{
    ChatCompletionRequestMessage,
    ChatCompletionRequestUserMessageArgs,
    ChatCompletionRequestSystemMessageArgs,
    CreateChatCompletionRequestArgs,
};
use async_openai::Client;
use tokio::io::{self, AsyncBufReadExt};
use anyhow::Result;
use crate::chatbots::types::*;


impl CharacterSession {
    pub fn new(name: &str, persona: &str) -> Self {
        Self {
            name: name.to_string(),
            persona: persona.to_string(),
            history: Vec::new(),
        }
    }

    async fn handle_message(&mut self, user_input: &str, model: &str) -> Result<String> {
        self.history.push(Message {
            role: "user".to_string(),
            content: user_input.to_string(),
        });

        // Convert conversation into OpenAI message list
        let mut messages: Vec<ChatCompletionRequestMessage> = Vec::new();

        // Persona system message
        messages.push(
            ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(format!(
                        "You are {} — a {}.\n\
                 Stay fully in character at all times. Never break the fourth wall, no matter what.\n\
                 Respond briefly and naturally (1–3 short sentences max).\
                 Avoid explanations, disclaimers, or meta-comments.",
                        self.name, self.persona
                    ))
                    .build()?,
            ),
        );


        for m in &self.history {
            match m.role.as_str() {
                "user" => messages.push(ChatCompletionRequestMessage::User(
                    ChatCompletionRequestUserMessageArgs::default()
                        .content(m.content.clone())
                        .build()?,
                )),
                "assistant" => messages.push(ChatCompletionRequestMessage::Assistant(
                    async_openai::types::ChatCompletionRequestAssistantMessageArgs::default()
                        .content(m.content.clone())
                        .build()?,
                )),
                _ => {}
            }
        }

        // API call
        let client = Client::new();
        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages(messages)
            .build()?;

        let response = client.chat().create(request).await?;
        let reply = response
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .unwrap_or_else(|| "[No reply]".to_string());

        self.history.push(Message {
            role: "assistant".to_string(),
            content: reply.clone(),
        });

        Ok(reply)
    }
}

impl ChatManager {
    pub fn new(model: &str) -> Self {
        Self {
            sessions: HashMap::new(),
            active: None,
            model: model.to_string(),
        }
    }

    pub fn add_character(&mut self, id: &str, persona: &str) {
        self.sessions
            .insert(id.to_string(), CharacterSession::new(id, persona));
    }

    pub fn switch_to(&mut self, id: &str) {
        if self.sessions.contains_key(id) {
            self.active = Some(id.to_string());
            println!("Switched to chat with '{}'.", id);
        } else {
            println!("No character with id '{}'", id);
        }
    }

    pub async fn send_message(&mut self, msg: &str) {
        if let Some(id) = &self.active {
            if let Some(session) = self.sessions.get_mut(id) {
                match session.handle_message(msg, &self.model).await {
                    Ok(reply) => println!("{}: {}\n", id, reply),
                    Err(e) => eprintln!("Error from {}: {:?}", id, e),
                }
            }
        } else {
            println!("No active character!");
        }
    }
}

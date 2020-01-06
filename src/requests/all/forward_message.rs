use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, Message},
    Bot,
};

/// Use this method to forward messages of any kind. On success, the sent
/// Message is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct ForwardMessage<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    chat_id: ChatId,
    /// Unique identifier for the chat where the original message was sent (or
    /// channel username in the format @channelusername)
    from_chat_id: ChatId,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    disable_notification: Option<bool>,
    /// Message identifier in the chat specified in from_chat_id
    message_id: i32,
}

#[async_trait::async_trait]
impl Request for ForwardMessage<'_> {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "forwardMessage",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> ForwardMessage<'a> {
    pub(crate) fn new<C, F>(
        bot: &'a Bot,
        chat_id: C,
        from_chat_id: F,
        message_id: i32,
    ) -> Self
    where
        C: Into<ChatId>,
        F: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        let from_chat_id = from_chat_id.into();
        Self {
            bot,
            chat_id,
            from_chat_id,
            message_id,
            disable_notification: None,
        }
    }

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.from_chat_id = val.into();
        self
    }

    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    pub fn message_id(mut self, val: i32) -> Self {
        self.message_id = val;
        self
    }
}
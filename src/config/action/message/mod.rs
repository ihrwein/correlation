use handlebars::{
    self,
    Handlebars,
    RenderError,
    Template
};
use rustc_serialize::json::ToJson;
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::fmt;

use message::{
    Message,
    MessageBuilder
};
use super::ActionType;

mod deser;
mod builder;

pub use self::builder::MessageActionBuilder;

#[derive(Clone)]
pub struct MessageAction {
    uuid: String,
    name: Option<String>,
    message: Template,
    values: BTreeMap<String, String>
}

impl MessageAction {
    pub fn uuid(&self) -> &String {
        &self.uuid
    }
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
    pub fn message(&self) -> &Template {
        &self.message
    }
    pub fn values(&self) -> &BTreeMap<String, String> {
        &self.values
    }
    pub fn render_message<T>(&self, data: T) -> Result<Message, RenderError> where T: ToJson {

    }
}

impl PartialEq for MessageAction {
    fn eq(&self, other: &MessageAction) -> bool {
        self.uuid() == other.uuid() &&
        self.name() == other.name() &&
        self.message().to_string() == other.message().to_string() &&
        self.values() == other.values()
    }
}

impl Eq for MessageAction {}

impl From<MessageAction> for super::ActionType {
    fn from(action: MessageAction) -> super::ActionType {
        super::ActionType::Message(action)
    }
}

impl fmt::Debug for MessageAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MessageAction")
            .field("uuid", self.uuid())
            .field("name", &self.name())
            .field("message", &self.message().to_string())
            .field("values", self.values())
            .finish();
    }
}

impl<'a> From<&'a MessageAction> for Message {
    fn from(action: &'a MessageAction) -> Message {
        let name = action.name().map(|name| name.borrow());
        MessageBuilder::new(action.uuid(), action.message())
                        .name(name)
                        .values(action.values().clone())
                        .build()
    }
}

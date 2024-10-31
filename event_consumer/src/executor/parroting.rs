use crate::executor::{execute_error, Executor};
use crate::line::messaging_api::reply_simple_message;
use crate::line::messaging_api::request::event::Event;
use crate::line::messaging_api::request::event::message::Message;
use crate::line::messaging_api::request::event::message::MessageEvent;

pub struct Parroting {}

impl Executor for Parroting {
    async fn execute(&self, event: &Event) -> Result<(), lambda_runtime::Error> {
        if let Event::Message(messageEvent) = event {
            let MessageEvent { reply_token, message, .. } = messageEvent;
            if let Message::Text { text, .. } = message {
                reply_simple_message(reply_token, text).await?
            }
        }

        log::debug!("Parroting requires Text Message Event.");
        Ok(())
    }
}
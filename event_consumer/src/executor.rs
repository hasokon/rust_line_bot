pub mod parroting;

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::line::messaging_api::request::event::Event;

#[derive(Debug)]
pub struct ExecuteError {
    pub reason: String,
}

impl Display for ExecuteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl Error for ExecuteError {}

fn execute_error<T: ToString>(s: T) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    Err(Box::new(ExecuteError {
        reason: s.to_string()
    }))
}

pub trait Executor {
    fn should_process(&self, event: &Event) -> bool;

    async fn execute(&self, event: &Event) -> Result<(), lambda_runtime::Error>;
}

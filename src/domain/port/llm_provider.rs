use crate::domain::model::input_item::InputItem;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

pub type LlmProviderError = Box<dyn Error + Send + Sync>;

pub type LlmProviderFuture<'a> =
    Pin<Box<dyn Future<Output = Result<Vec<InputItem>, LlmProviderError>> + Send + 'a>>;

pub trait LlmProvider: Send + Sync {
    fn response<'a>(
        &'a self,
        model: &'a str,
        instruction: &'a str,
        input: Vec<InputItem>,
    ) -> LlmProviderFuture<'a>;
}

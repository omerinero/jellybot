use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplyMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboard>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineKeyboard {
    pub text: String,
    pub url: String,
}


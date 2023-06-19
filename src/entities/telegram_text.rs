use crate::env_loader::TELEGRAM_CHAT_ID_KEY;
use crate::traits::telegram::TelegramJSON;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TelegramText {
    pub text: String,
    pub parse_mode: String,
    pub disable_web_page_preview: bool,
    pub disable_notification: bool,
    pub chat_id: String,
}

impl TelegramText {
    #[allow(dead_code)]
    pub fn builder() -> TelegramTextBuilder {
        TelegramTextBuilder::default()
    }
}

#[derive(Default)]
pub struct TelegramTextBuilder {
    text: String,
    parse_mode: String,
    disable_web_page_preview: bool,
    disable_notification: bool,
    chat_id: String,
}

impl TelegramTextBuilder {
    pub fn new() -> Self {
        Self {
            text: "".to_string(),
            parse_mode: "Markdown".to_string(),
            disable_web_page_preview: true,
            disable_notification: false,
            chat_id: TELEGRAM_CHAT_ID_KEY(),
        }
    }

    pub fn with_text(mut self, value: String) -> Self {
        self.text = value;
        self
    }

    pub fn build(self) -> TelegramText {
        TelegramText {
            text: self.text,
            parse_mode: self.parse_mode,
            disable_web_page_preview: self.disable_web_page_preview,
            disable_notification: self.disable_notification,
            chat_id: self.chat_id,
        }
    }
}

impl TelegramJSON for TelegramText {
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

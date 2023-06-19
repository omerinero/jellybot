use rust_i18n::t;
use crate::traits::telegram::TelegramJSON;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::entities::reply_markup::{InlineKeyboard, ReplyMarkup};
use crate::env_loader::TELEGRAM_CHAT_ID_KEY;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TelegramPhoto {
    pub caption: String,
    pub chat_id: String,
    pub photo: String,
    pub parse_mode: String,
    pub reply_markup: ReplyMarkup,
}

impl TelegramPhoto {
    #[allow(dead_code)]
    pub fn builder() -> TelegramPhotoBuilder {
        TelegramPhotoBuilder::default()
    }
}

#[derive(Default)]
pub struct TelegramPhotoBuilder {
    caption: String,
    chat_id: String,
    photo: String,
    parse_mode: String,
    link_detail: String,
}

impl TelegramPhotoBuilder {
    pub fn new() -> Self {
        Self {
            caption: "".to_string(),
            chat_id: TELEGRAM_CHAT_ID_KEY(),
            photo: "".to_string(),
            parse_mode: "Markdown".to_string(),
            link_detail: "".to_string(),
        }
    }

    pub fn with_caption(mut self, value: String) -> Self {
        self.caption = value;
        self
    }

    pub fn with_photo(mut self, value: String) -> Self {
        self.photo = value;
        self
    }

    pub fn with_link_detail(mut self, value: String) -> Self {
        self.link_detail = format!("https://www.imdb.com/title/{}", value);
        self
    }

    pub fn build(self) -> TelegramPhoto {
        let inlineKeyboard: InlineKeyboard = InlineKeyboard { text: t!("watch_trailer"), url: self.link_detail };
        let reply = ReplyMarkup { inline_keyboard: vec![vec![inlineKeyboard]] };

        TelegramPhoto {
            caption: self.caption,
            photo: self.photo,
            chat_id: self.chat_id,
            parse_mode: self.parse_mode,
            reply_markup: reply,
        }
    }
}

impl TelegramJSON for TelegramPhoto {
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

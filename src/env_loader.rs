use std::env;

pub fn check_env_variables() {
    TELEGRAM_BOT_API_KEY();
    TELEGRAM_CHAT_ID_KEY();
    JELLYFIN_API_KEY();
}

pub fn TELEGRAM_BOT_API_KEY() -> String {
    match env::var("TELEGRAM_BOT_API_KEY") {
        Ok(value) => value,
        Err(_) => {
            panic!("TELEGRAM_BOT_API_KEY not set")
        }
    }
}

pub fn TELEGRAM_CHAT_ID_KEY() -> String {
    match env::var("TELEGRAM_CHAT_ID_KEY") {
        Ok(value) => value,
        Err(_) => {
            panic!("TELEGRAM_CHAT_ID_KEY not set")
        }
    }
}

pub fn JELLYFIN_API_KEY() -> String {
    match env::var("JELLYFIN_API_KEY") {
        Ok(value) => value,
        Err(_) => {
            panic!("JELLYFIN_API_KEY not set")
        }
    }
}

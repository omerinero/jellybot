pub mod endpoints {
    #![allow(non_snake_case)]

    use rocket::serde::json::Json;

    use crate::entities::item::{Item, ItemType};
    use crate::entities::radarr_item::RadarrItem;
    use crate::entities::telegram_photo::TelegramPhotoBuilder;
    use crate::entities::telegram_text::TelegramTextBuilder;
    use crate::services::jellyfin::api;
    use crate::services::telegram::api::executeRequest;
    use crate::traits::telegram::TelegramJSON;

    #[post("/action", data = "<item>", format = "json")]
    pub async fn action(item: Json<Item>) {
        let itemBox = Box::new(item.0);

        match itemBox.itemType {
            ItemType::Movies | ItemType::Movie => createTelegramSendPhotoRequest(itemBox).await,
            ItemType::Season | ItemType::Seasons => createTelegramSendPhotoRequest(itemBox).await,
            ItemType::Episode | ItemType::Episodes => {
                createTelegramSendMessageRequest(itemBox).await
            }
            _ => {}
        };
    }

    #[post("/radarr", data = "<item>", format = "json")]
    pub async fn radarr(item: Json<RadarrItem>) {
        let _itemBox = Box::new(item.0);
    }

    async fn createTelegramSendMessageRequest(item: Box<Item>) {
        let body = TelegramTextBuilder::new()
            .with_text(
                t!("tv_show_available", name = item.seriesName, season = item.seasonNumber, episode_number = item.episodeNumber)
            )
            .build()
            .to_json();

        executeRequest(body, "sendMessage".to_string()).await;
    }

    async fn createTelegramSendPhotoRequest(itemBox: Box<Item>) {
        let name = match itemBox.itemType {
            ItemType::Season | ItemType::Seasons => itemBox.seriesName,
            _ => itemBox.name,
        };

        let body = TelegramPhotoBuilder::new()
            .with_photo(api::getPrimaryImage(&itemBox.serverUrl, &itemBox.id))
            .with_caption(t!("movie_available", name = name))
            .with_link_detail(itemBox.imdb)
            .build()
            .to_json();

        executeRequest(body, "sendPhoto".to_string()).await;
    }
}

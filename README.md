# jellybot
This is a personal project and to better understand Rust by putting into practice various skills.

The idea is simply to connect the Telegram chat with Jellyfin to notify the user if there is new content.

The bot exposes a local API where Jellyfin can make calls whenever there is new content. Jellybot will parse the content and make it pretty to send the message via Telegram.

This project uses Rocket, Reqwest and i18n among others.

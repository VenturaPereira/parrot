use serenity::{http::Http, model::channel::Message, utils::Colour};
use std::{sync::Arc, time::Duration};

pub async fn send_simple_message(http: &Arc<Http>, msg: &Message, content: &str) -> Message {
    msg.channel_id.send_message(http, |m| {
        m.embed(|e| e.description(content).colour(Colour::ORANGE))
    }).await.expect("Unable to send message")
}

pub fn get_human_readable_timestamp(duration: Duration) -> String {
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let hours = duration.as_secs() / 3600;

    let timestamp = if hours < 1 {
        format!("{}:{:02}", minutes, seconds)
    } else {
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    };

    timestamp
}

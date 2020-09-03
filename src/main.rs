mod wakatime;

use std::env;
use egg_mode::{Token, KeyPair};
use egg_mode::tweet::DraftTweet;
use crate::wakatime::{SummaryLanguage, SummaryCategory, SummaryEditor};

#[tokio::main]
async fn main() {
    let token = get_access_token();
    let summary = wakatime::WakaTime::new()
        .fetch_summaries()
        .await
        .data;

    let main_tweet = DraftTweet::new(format!("[WakaTime Report]\n\n\
    Total Coding Time: {}\n\
    Average Daily Coding Time: {}", summary.human_readable_total, summary.human_readable_daily_average))
        .send(&token)
        .await
        .expect("The tweet failed to be sent.")
        .response;

    let categories = create_draft_categories(summary.categories)
        .in_reply_to(main_tweet.id)
        .send(&token)
        .await
        .expect("The tweet failed to be sent.")
        .response;

    let editors = create_draft_editors(summary.editors)
        .in_reply_to(categories.id)
        .send(&token)
        .await
        .expect("The tweet failed to be sent.")
        .response;

    create_draft_languages(summary.languages)
        .in_reply_to(editors.id)
        .send(&token)
        .await
        .expect("The tweet failed to be sent.");
}

fn create_draft_categories(categories: Vec<SummaryCategory>) -> DraftTweet {
    let mut content: Vec<String> = Vec::new();

    content.push("[Top Categories]\n".to_string());

    categories
        .iter()
        .take(3)
        .for_each(|category| content.push(format!("{} : {}", category.name, category.text)));

    DraftTweet::new(content.join("\n"))
}

fn create_draft_editors(editors: Vec<SummaryEditor>) -> DraftTweet {
    let mut content: Vec<String> = Vec::new();

    content.push("[Top Editors]\n".to_string());

    editors
        .iter()
        .take(3)
        .for_each(|editor| content.push(format!("{} : {}", editor.name, editor.text)));

    DraftTweet::new(content.join("\n"))
}

fn create_draft_languages(languages: Vec<SummaryLanguage>) -> DraftTweet {
    let mut content: Vec<String> = Vec::new();

    content.push("[Top Languages]\n".to_string());

    languages
        .iter()
        .filter(|language| language.name != "Other")
        .take(3)
        .for_each(|language| content.push(format!("{} : {}", language.name, language.text)));

    DraftTweet::new(content.join("\n"))
}

fn get_access_token() -> Token {
    let consumer_key = env::var("TWITTER_CONSUMER_KEY")
        .expect("The Twitter API consumer key is not set in the environment variable \"TWITTER_CONSUMER_KEY\".");
    let consumer_key_secret = env::var("TWITTER_CONSUMER_KEY_SECRET")
        .expect("The Twitter API consumer secret key is not set in the environment variable \"TWITTER_CONSUMER_KEY_SECRET\".");
    let access_token = env::var("TWITTER_ACCESS_TOKEN")
        .expect("The Twitter API access token is not set in the environment variable \"TWITTER_ACCESS_TOKEN\".");
    let access_token_secret = env::var("TWITTER_ACCESS_TOKEN_SECRET")
        .expect("The Twitter API access secret token is not set in the environment variable \"TWITTER_ACCESS_TOKEN_SECRET\".");

    let consumer = KeyPair::new(consumer_key, consumer_key_secret);
    let access = KeyPair::new(access_token, access_token_secret);

    Token::Access {
        consumer,
        access,
    }
}

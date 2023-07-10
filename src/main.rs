mod date;

use std::{env, error::Error};

use chrono::prelude::*;
use date::RepublicanDate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    // get today's date and the theme data
    let today: RepublicanDate = Utc::now().date_naive().into();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("themes.csv")?;
    let record = reader
        .records()
        .nth(today.date as usize)
        .expect("csv did not have the expected date row")?;
    let theme = &record[0];
    let image_desc_url = &record[1];
    let image_src_url = &record[2];

    // download image
    let client = reqwest::Client::builder()
        .user_agent(format!(
            "primidi cohost bot ({})",
            &env::var("COHOST_EMAIL")?
        ))
        .build()?;

    let response = client.get(image_src_url).send().await?;
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let image_data = response.bytes().await?;
    assert!(image_data.len() <= 5_000_000);

    // log in to cohost
    let session = eggbug::Session::login(&env::var("COHOST_EMAIL")?, &env::var("COHOST_PASSWORD")?)
        .await
        .expect("Could not log into cohost.");

    // make the post
    let date: RepublicanDate = NaiveDate::from_ymd_opt(2022, 9, 23).unwrap().into();
    let title = if theme.is_empty() {
        format!("Today is {date}.")
    } else {
        format!("Today is {date}, celebrating {theme}.")
    };
    // TODO: alt text format for celebration days
    let alt_text = format!("Illustration of {theme}");
    let mut tags = vec!["french republican calendar".to_owned()];
    if !theme.is_empty() {
        tags.push(theme.strip_prefix("the ").unwrap_or(theme).to_owned());
    }

    let mut post = eggbug::Post {
        headline: title,
        markdown: format!(
            r#"<div style="display:flex;justify-content:center;font-size:0.8rem"><a href="{image_desc_url}">[image source]</a></div>"#
        ),
        attachments: vec![eggbug::Attachment::new(
            image_data,
            "image.jpg".into(),
            "image/jpeg".into(),
            /* width */ None,
            /* height */ None,
        )
        .with_alt_text(alt_text)],
        tags,
        ..Default::default()
    };

    session
        .create_post(&env::var("COHOST_PAGE")?, &mut post)
        .await?;
    Ok(())
}

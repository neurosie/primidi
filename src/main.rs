mod date;

use std::{env, error::Error};

use chrono::prelude::*;
use date::RepublicanDate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    // download image
    let client = reqwest::Client::builder()
        .user_agent(format!(
            "primidi cohost bot ({})",
            &env::var("COHOST_EMAIL")?
        ))
        .build()?;
    let image_url = "https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Crocus_Sativus_-_Zaffrano_domestico_-_Saffron._%28saffron_crocus%3B_field_crocus%29_%28NYPL_b14444147-1130719%29.tiff/lossy-page1-1521px-Crocus_Sativus_-_Zaffrano_domestico_-_Saffron._%28saffron_crocus%3B_field_crocus%29_%28NYPL_b14444147-1130719%29.tiff.jpg";

    let response = client.get(image_url).send().await?;
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let image_data = response.bytes().await?;

    assert!(image_data.len() <= 5_000_000);

    // log in to cohost
    let session = eggbug::Session::login(&env::var("COHOST_EMAIL")?, &env::var("COHOST_PASSWORD")?)
        .await
        .expect("Could not log into cohost.");

    // make the post
    let date: RepublicanDate = NaiveDate::from_ymd_opt(2022, 9, 23).unwrap().into();
    let theme = "saffron";
    let title = format!("Today is {date}, celebrating {theme}.");
    let alt_text = format!("Illustration of {theme}");

    let mut post = eggbug::Post {
        headline: title,
        markdown: format!("[(image source)]({image_url})"),
        attachments: vec![eggbug::Attachment::new(
            image_data,
            "image.jpg".into(),
            "image/jpeg".into(),
            /* width */ None,
            /* height */ None,
        )
        .with_alt_text(alt_text)],
        draft: true,
        ..Default::default()
    };

    session
        .create_post(&env::var("COHOST_PAGE")?, &mut post)
        .await?;
    Ok(())
}

mod args;
mod utils;

use std::env;
use std::path::Path;

use args::Args;
use clap::Parser;
use dotenv::dotenv;
use lettre::{
    message::header, transport::smtp::authentication::Credentials, Message, SmtpTransport,
    Transport,
};
use utils::{get_config, get_template};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args = Args::parse();

    let config = get_config(&Path::new(&args.config))?;

    let email_username = env::var("EMAIL_USERNAME")?;
    let email_pass = env::var("EMAIL_PASS")?;
    let email_from = env::var("EMAIL_FROM")?;

    let to = config.to.expect("Faild: to is missing");
    let subject = config
        .subject
        .unwrap_or(String::from("Message subject #marco mail-sender"));
    let body = match config.html {
        Some(ref html) => get_template(&Path::new(html))?,
        None => config.body.unwrap_or(String::from("Hello")),
    };

    let email = Message::builder()
        .from(email_from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .header(header::ContentType::TEXT_HTML)
        .body(body)?;

    let smtp_transport = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(Credentials::new(email_username, email_pass))
        .build();

    smtp_transport.send(&email)?;

    Ok(())
}

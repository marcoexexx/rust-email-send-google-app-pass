use std::env;

use dotenv::dotenv;
use lettre::{Message, SmtpTransport, transport::smtp::authentication::Credentials, Transport};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let email_username = env::var("EMAIL_USERNAME")?;
    let email_pass = env::var("EMAIL_PASS")?;

    println!("{} {}", email_pass, email_username);

    let email = Message::builder()
        .from("toyko2001@gmail.com".parse().unwrap())
        .to("toyko2001@gmail.com".parse().unwrap())
        .subject("Some subject")
        .body(String::from("Some body"))?;

    let smtp_transport = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(Credentials::new(email_username, email_pass))
        .build();

    smtp_transport.send(&email)?;

    Ok(())
}


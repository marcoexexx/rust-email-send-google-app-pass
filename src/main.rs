use std::env;

use dotenv::dotenv;
use lazy_static::lazy_static;
use lettre::{
    message::header, transport::smtp::authentication::Credentials, Message, SmtpTransport,
    Transport,
};
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let email_username = env::var("EMAIL_USERNAME")?;
    let email_pass = env::var("EMAIL_PASS")?;
    let email_from = env::var("EMAIL_FROM")?;

    let html = TEMPLATES.render("subject.html", &Context::new()).unwrap();

    let email = Message::builder()
        .from(email_from.parse().unwrap())
        .to("toyko2001@gmail.com".parse().unwrap())
        .subject("Some subject")
        .header(header::ContentType::TEXT_HTML)
        .body(html)?;

    let smtp_transport = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(Credentials::new(email_username, email_pass))
        .build();

    smtp_transport.send(&email)?;

    Ok(())
}

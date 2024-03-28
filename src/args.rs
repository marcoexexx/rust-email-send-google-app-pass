#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub to: Option<String>,
    pub subject: Option<String>,
    pub html: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub config: String,
}

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the directory containing the image index
    #[arg(long, value_name = "PATH")]
    pub dir: std::path::PathBuf,
    /// Alt text to use for posted images
    #[arg(long = "alt", value_name = "ALT")]
    pub alt_text: Option<String>,
    /// Tags to add to each post (semi-colon-separated)
    #[arg(long, value_delimiter = ';', value_name = "TAG;...")]
    pub tags: Option<Vec<String>>,
}

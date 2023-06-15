mod image_parser;
mod video_parser;

use clap::{Parser, ValueEnum};
use eyre::{Result};

#[derive(Debug, Clone, ValueEnum)]
pub enum FileType {
    Image,
    Video,
}

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    file_type: FileType,

    #[arg(short, long)]
    path: String,

    #[arg(short, long, default_value = Option::None)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.file_type {
        FileType::Image => {
            image_parser::parse_image(args.path.as_str(), args.output)?;
        }
        FileType::Video => {}
    }

    Ok(())
}

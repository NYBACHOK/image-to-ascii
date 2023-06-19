mod character_set;
mod image_parser;
mod resize;
mod video_parser;

use clap::{Parser, ValueEnum};
use eyre::Result;

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
    let args = Args::try_parse().unwrap_or(Args{
        file_type: FileType::Video,
        path: "~/image.mp4".to_string(),
        output: None,
    });

    match args.file_type {
        FileType::Image => {
            image_parser::parse_image(args.path.as_str(), args.output)?;
        }
        FileType::Video => {
            video_parser::start_playing(args.path.as_str())?;
        }
    }

    Ok(())
}

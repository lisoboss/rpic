mod config;
mod image;
mod oss;

use clap::{Parser, ValueHint};
use std::path::PathBuf;

use image::Image;
use oss::put_webp;

#[derive(Parser, Debug)]
#[command(name = "rpic")]
#[command(about = "Convert images to WebP and upload to OSS", long_about = None)]
struct Args {
    /// Image files to convert and upload
    #[arg(value_hint = ValueHint::FilePath, required = true)]
    images: Vec<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Args { images } = Args::parse();

    let mut urls = Vec::with_capacity(images.len());
    for img_path in images {
        let img = Image::new(img_path)?;
        let webp_data = img.to_webp();
        let url = put_webp(&webp_data).await?;
        urls.push(url);
    }

    println!("Upload Success:\n{}", urls.join("\n"));
    Ok(())
}

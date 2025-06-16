use anyhow::Result;
use image::{ImageBuffer, ImageReader, Rgba};
use oss::put_webp;
use std::env;
use std::path::PathBuf;
use webp::{Encoder, WebPMemory};

mod config;
mod oss;

#[derive(Debug)]
struct ImgData {
    rgba: ImageBuffer<Rgba<u8>, Vec<u8>>,
    width: u32,
    height: u32,
}

fn read(img_path: PathBuf) -> Result<ImgData> {
    let img = ImageReader::open(img_path)?.decode()?;

    Ok(ImgData {
        rgba: img.to_rgba8(),
        width: img.width(),
        height: img.height(),
    })
}

fn to_webp(img: ImgData) -> WebPMemory {
    // 使用 webp crate 进行编码
    let encoder = Encoder::from_rgba(&img.rgba, img.width, img.height);
    // 设置压缩质量，范围 0.0 ~ 100.0（这里设置为 75.0）
    encoder.encode(75.0)
}

#[tokio::main]
async fn main() -> Result<()> {
    // 从命令行参数获取输入和输出文件路径
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("用法: {} <图片路径1> <图片路径2> <图片路径3> ...", args[0]);
        return Ok(());
    }

    let mut urls = Vec::with_capacity(args.len() - 1);
    for img_path in args.split_off(1) {
        let img = read(img_path.into())?;
        let webp_data = to_webp(img);
        let url = put_webp(&webp_data).await?;
        urls.push(url);
    }

    println!("Upload Success:\n{}", urls.join("\n"));
    Ok(())
}

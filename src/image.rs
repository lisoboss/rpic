use anyhow::Result;
use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, ImageBuffer, ImageReader, Rgba};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use webp::{AnimEncoder, Encoder, WebPConfig, WebPMemory};

#[derive(Debug)]
struct Frame {
    rgba: Vec<u8>,
    width: u32,
    height: u32,
    timestamp: i32,
}

#[derive(Debug)]
enum ImgData {
    Static(Frame),
    Animated {
        frames: Vec<Frame>,
        width: u32,
        height: u32,
    },
}

pub struct Image {
    data: ImgData,
}

impl Image {
    pub fn new(img_path: PathBuf) -> Result<Self> {
        let data = Self::read(img_path)?;
        Ok(Self { data })
    }

    pub fn to_webp(&self) -> WebPMemory {
        match &self.data {
            ImgData::Static(Frame {
                rgba,
                width,
                height,
                ..
            }) => {
                // 使用原有的静态 WebP 编码
                let encoder = Encoder::from_rgba(rgba, *width, *height);
                encoder.encode(75.0)
            }
            ImgData::Animated {
                frames,
                width,
                height,
            } => {
                // 使用动画 WebP 编码
                let mut config = WebPConfig::new().expect("Failed to create WebP config");
                config.method = 4; // 最佳压缩方法

                let mut anim_encoder = AnimEncoder::new(*width, *height, &config);

                // 创建所有帧并收集
                let webp_frames: Vec<_> = frames
                    .iter()
                    .map(
                        |Frame {
                             rgba,
                             width,
                             height,
                             timestamp,
                         }| {
                            webp::AnimFrame::from_rgba(&rgba, *width, *height, *timestamp)
                        },
                    )
                    .collect();

                // 添加所有帧
                for frame in webp_frames {
                    anim_encoder.add_frame(frame);
                }

                // 编码动画 WebP
                anim_encoder.encode()
            }
        }
    }

    fn read(img_path: PathBuf) -> Result<ImgData> {
        // 尝试用 GIF 解码器读取
        let file = File::open(&img_path)?;
        let reader = BufReader::new(file);

        if let Ok(decoder) = GifDecoder::new(reader) {
            let frames = decoder.into_frames();
            let frame_vec: Vec<image::Frame> = frames.collect::<Result<Vec<_>, _>>()?;

            // 只有多帧才是动画
            if frame_vec.len() > 1 {
                let (width, height) = (
                    frame_vec[0].buffer().width(),
                    frame_vec[0].buffer().height(),
                );

                let mut frames_data = Vec::new();

                for frame in frame_vec {
                    let delay = frame.delay().numer_denom_ms().0 as i32;
                    let rgba: ImageBuffer<Rgba<u8>, Vec<u8>> = frame.into_buffer();
                    let (w, h) = (rgba.width(), rgba.height());

                    frames_data.push(Frame {
                        rgba: rgba.into_raw(),
                        width: w,
                        height: h,
                        timestamp: delay,
                    });
                }

                return Ok(ImgData::Animated {
                    frames: frames_data,
                    width,
                    height,
                });
            }
        }

        // 非动画格式，按原有逻辑处理
        let img = ImageReader::open(&img_path)?.decode()?;
        let rgba = img.to_rgba8().into_raw();
        let (width, height) = (img.width(), img.height());

        Ok(ImgData::Static(Frame {
            rgba,
            width,
            height,
            timestamp: 0, // 静态图没有延迟
        }))
    }
}

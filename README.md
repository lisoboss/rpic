# rpic

`rpic` 是一个命令行工具，用于将本地图片同步上传至阿里云 OSS，并返回可访问的图片 URL 列表。适合图片备份、图床上传等自动化需求。

## ✨ 特性

- 支持多图片批量上传
- 自动生成公开访问链接
- 可集成至脚本、CI/CD、博客构建流程等场景
- 简洁快速，无需多余依赖

## 📦 安装

你可以通过构建源码安装：

```bash
git clone https://github.com/lisoboss/rpic.git
cd rpic
cargo build --release
```

Or

```bash
cargo install --locked --git https://github.com/lisoboss/rpic.git
```

## OSS 配置说明

本项目支持从以下路径自动加载 OSS 配置（按优先级顺序）：

1. 环境变量
2. 配置文件：
    - `/etc/rpic/config.toml`
    - `~/.config/rpic/config.toml`

### 环境变量方式

你可以设置以下环境变量来指定 OSS 配置：

```bash
export RPIC_OSS_ACCESS_KEY_ID="LTAIxxxxxxxxxxxxxx"
export RPIC_OSS_ACCESS_KEY_SECRET="xxxxxxxxxxxxxxxxxxxxxxxxx"
export RPIC_OSS_BUCKET="your-bucket-name"
export RPIC_OSS_ENDPOINT="oss-cn-your-region.aliyuncs.com"
```

### 配置文件方式

如果没有设置环境变量，程序将尝试加载以下位置的配置文件：

- `/etc/rpic/config.toml`
- `~/.config/rpic/config.toml`

```toml
[oss]
access_key_id = "LTAIxxxxxxxxxxxxxx"
access_key_secret = "xxxxxxxxxxxxxxxxxxxxxxxxx"
bucket = "your-bucket-name"
endpoint = "oss-cn-your-region.aliyuncs.com"
```

## 作用

主要配合 `Typora` app 的图片保存使用

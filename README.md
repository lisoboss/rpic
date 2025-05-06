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
git clone https://github.com/your-name/rpic.git
cd rpic
cargo build --release
```

## 作用

主要配合 `Typora` app 的图片保存使用
